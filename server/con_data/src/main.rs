use entity::model::db::{SubTypeValues, Well};
use mongodb::{Client, Database};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

// internal
use crate::app_errors::{AppError, AppResult};
use common::{constants::REASTART_APP_SEC, env_helper::EnvHelper};
use db::{dberror, mongo::read_all_wells};

mod app_errors;

extern crate common;
extern crate config;

const BANNER: &str = include_str!("../assets/banner.txt");

#[tokio::main]
async fn main() {
    common::file_helper::change_working_directory();

    let well_id = Arc::new(EnvHelper::get_well_id());
    let mongo_address = Arc::new(EnvHelper::get_mongo_address());
    let redis_address = Arc::new(EnvHelper::get_redis_address());
    let activemq_address = Arc::new(EnvHelper::get_amq_address());

    logger::init_with_name(&format!("con_data_{}.log", &well_id));

    logger::info(BANNER);
    logger::info("Con Data started!");
    logger::info(format!("Well ID is :{}", &well_id).as_str());
    logger::info(format!("DB connection string:'{}'", &mongo_address).as_str());
    logger::info(format!("Redis connection string:'{}'", &redis_address).as_str());
    logger::info(format!("AMQ connection string:'{}'", &activemq_address).as_str());

    print_info();

    loop {
        match run_application(&mongo_address, &well_id).await {
            Ok(_) => {
                logger::info("Application finished normally");
                break;
            }

            Err(err) => {
                logger::error(format!("Application failed: {err}").as_str());
                logger::info(format!("Restarting in {REASTART_APP_SEC} seconds...").as_str());

                sleep(Duration::from_secs(REASTART_APP_SEC)).await;
            }
        }
    }
}

fn print_info() {
    // libs
    logger::info(&format!("entity version is: {}", entity::get_version()));
    logger::info(&format!("common version is: {}", common::get_lib_version()));
    logger::info(&format!("config version is: {}", config::get_version()));
    logger::info(&format!("logger version is: {}", logger::get_version()));

    // directory ifo
    logger::info(&format!(
        "Working directory is: {}",
        common::file_helper::current_path().to_str().unwrap()
    ));
    logger::info(&format!(
        "Configure directory is: {}",
        common::file_helper::config_path().to_str().unwrap()
    ));
}

async fn run_application(mongo_address: &Arc<String>, well_id: &Arc<String>) -> AppResult<()> {
    let _ = CancellationToken::new();

    let (client, database) = match init_db(&mongo_address).await {
        Ok((client, db)) => (client, db),
        Err(err) => {
            logger::error(&format!("{err}"));

            return Err(err.into());
        }
    };

    let well_data = match fetch_well(database, &well_id).await {
        Ok(well) => well,
        Err(err) => {
            logger::error(&format!("{err}"));

            return Err(err.into());
        }
    };

    let subtype_value = well_data
        .server_configure
        .mq_client_configure
        .sub_type
        .value;
    if subtype_value != (SubTypeValues::AMQTextConsumer as i32) {
        let error_msg = format!(
            "DANGER ==> This program is not supporing any subtype except for AMQTextConsumer(32), current subtype is '{subtype_value}'"
        );
        logger::error(error_msg.as_str());
        return Err(AppError::Text(error_msg));
    }

    logger::info(&format!(
        "Well #{} is fetched from db ({})",
        well_data.id,
        well_data.get_topic_name()
    ));

    client.shutdown().await;

    Ok(())
}

async fn init_db(connection_str: &Arc<String>) -> dberror::Result<(Client, Database)> {
    logger::info("preparing database...");
    let (client, database) = db::mongo::init_with_connection_string(connection_str).await?;

    Ok((client, database))
}

async fn fetch_well(database: Database, well_id: &Arc<String>) -> dberror::Result<Arc<&Well>> {
    let well = read_all_wells(database)
        .await?
        .iter()
        .find(|well| well.id == well_id.to_string());

    if let Some(well) = well {
        return Ok(Arc::new(well));
    } else {
        return Err(dberror::DbError::TextError(
            "Error in getting well from db".into(),
        ));
    }
}
