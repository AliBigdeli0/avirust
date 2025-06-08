// #![deny(warnings)]

extern crate common;
extern crate db;
extern crate entity;
extern crate logger;
extern crate mq;

mod serverdbargs;
mod serverdberrors;

use clap::Parser;
use common::file_helper::CONFIG_EXTENSION;
use serverdbargs::ServerDbArgs;
use serverdberrors::ServerDbError;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> serverdberrors::Result<()> {
    let args = ServerDbArgs::parse();

    common::file_helper::change_working_directory();

    ///////////////////////////////////
    // database actions
    let db_tuple = read_well_fromdb(args.clone()).await?;
    let well = db_tuple.0;
    let database_client = db_tuple.2;
    let wellname = well.name.to_string();

    logger::init_with_name(format!("{}{}", wellname, CONFIG_EXTENSION).as_str());
    logger::info("loging is initiated");
    logger::info("going to start server");

    ///////////////////////////////////
    // print entered data
    print_info(args.clone(), well.clone());

    ///////////////////////////////////
    // run tasks
    let work_task = tokio::spawn(async move {
        loop {
            ///////////////////////////////////
            // mq init
            let mq_well = well.clone();
            let mq_name = mq_well.server_configure.mq_client_configure.uri.value;
            let mq_address = mq_well
                .server_configure
                .mq_client_configure
                .mq_offline_name
                .value;

            let mq_instance = mq::mq::init_session(mq_name, mq_address, mq::mq::TypeOfMQ::Stomp)
                .await
                .map_err(|e| {
                    serverdberrors::ServerDbError::TextErr(format!(
                        "error in initiate mq instance: {:?}",
                        e
                    ))
                });

            match mq_instance {
                Ok(mut value) => {
                    let mq = value.as_mut();
                    let connection_result = mq.connect_to_mq().await;

                    if let Ok(_) = connection_result {
                        logger::info("Connected to the mq");
                    } else if let Err(err) = connection_result {
                        logger::error(&format!("Error in initiate mq instance: {:?}", err))
                    }
                }
                Err(err) => logger::error(&format!("Error in initiate mq instance: {:?}", err)),
            }

            // run instance
            // match mq_instance {
            //     Ok(value) => {
            //         let mut mq = *value;

            //         let client = match mq.connect().await {
            //             Ok(cli) => cli,
            //             Err(er) => {
            //                 logger::error(&format!("error create client '{:?}'", er));
            //                 return; //exit match
            //             }
            //         };

            //         logger::info("server is connected to the MQ");

            //         let mut receiver = match mq.subscribe(&client).await {
            //             Ok(rec) => rec,
            //             Err(er) => {
            //                 logger::error(&format!("error create client '{:?}'", er));
            //                 return; // exit match
            //             }
            //         };

            //         logger::info("server is subscribed to the message");

            //         loop {
            //             // mq.send("my code in times ".into(), &client).await.unwrap();

            //             let result = mq.receive(&mut receiver).await;

            //             match result {
            //                 Ok(_) => { /*nothing*/ }
            //                 Err(e) => {
            //                     logger::error(&format!("error in get frame : {:?}", e));
            //                 }
            //             }
            //         }
            //     }
            //     Err(e) => {
            //         logger::error(&format!("error in initiate mq instance: {:?}", e));
            //     }
            // };

            // finish part
            sleep(Duration::from_secs(3)).await;
            logger::error("==> going to reset the main loop <==");
        }
    });

    let signals_task = tokio::spawn(async {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                logger::info("crtl-c received.");
                return;
            }
            Err(e) => logger::error(&e.to_string()),
        }
    });

    tokio::select! {
        _ = work_task => {},
        _ = signals_task => {},
    }

    ///////////////////////////////////
    // close program
    logger::error("DANGER ==> server closed !<==");

    database_client.shutdown_immediate().await;
    logger::error("database closed!");

    Ok(())
}

fn print_info(args: ServerDbArgs, well: entity::model::db::Well) {
    logger::info(format!("Well ID '{}'", args.wellid).as_str());
    logger::info(format!("MongoDb connection '{}'", args.connectionstring).as_str());
    logger::info(format!("Well name Is : '{}'", well.name).as_str());
    logger::info(
        format!(
            "MQ address '{}'",
            well.server_configure.mq_client_configure.uri.value
        )
        .as_str(),
    );
    logger::info(
        format!(
            "MQ name is : '{}'",
            well.server_configure
                .mq_client_configure
                .mq_offline_name
                .value
        )
        .as_str(),
    );
}

async fn read_well_fromdb<'a>(
    args: serverdbargs::ServerDbArgs,
) -> serverdberrors::Result<(
    &'a entity::model::db::Well,
    mongodb::Database,
    mongodb::Client,
)> {
    let (client, db) = db::mongo::init_with_connection_string(&args.connectionstring)
        .await
        .map_err(|_| {
            serverdberrors::ServerDbError::TextErr(
                format!("error in database connection").to_string(),
            )
        })?;

    let all_wells = db::mongo::read_all_wells(db.clone())
        .await
        .map_err(|_| serverdberrors::ServerDbError::TextErr("error in read all wells".into()))?;

    let well_iter = all_wells.iter().find(|well| well.id == args.wellid);

    if well_iter.is_none() {
        println!("error in get well from db with id : {}", args.wellid);
        return Err(ServerDbError::TextErr(
            "selected well does not exist in well".into(),
        ));
    }

    let well = well_iter.ok_or(serverdberrors::ServerDbError::TextErr(
        "error in convert to well".into(),
    ))?;

    Ok((well, db, client))
}
