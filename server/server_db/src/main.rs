#![deny(warnings)]

mod serverdbargs;

use clap::Parser;
use common::file_helper::CONFIG_EXTENSION;
use serverdbargs::ServerDbArgs;
use std::time::Duration;
use tokio::time::sleep;

extern crate common;
extern crate logger;

#[tokio::main]
async fn main() {
    let args = ServerDbArgs::parse();
    common::file_helper::change_working_directory();

    logger::init_with_name(format!("{}{}", args.wellname.trim(), CONFIG_EXTENSION).as_str());

    logger::info("server started");
    print_pargs_info(args.clone());
    let work_task = tokio::spawn(async {
        loop {
            println!("application is running");
            sleep(Duration::from_secs(3)).await;
        }
    });

    let signals_task = tokio::spawn(async {
        match tokio::signal::ctrl_c().await {
            Ok(()) => println!("crtl-c received... now close resources..."),
            Err(e) => logger::error(&e.to_string()),
        }
    });

    tokio::select! {
        _ = work_task => {},
        _ = signals_task => {},
    }

    logger::error("DANGER ==> server closed !<==");
}

fn print_pargs_info(args: ServerDbArgs) {
    logger::info(format!("Well ID '{}'", args.wellid).as_str());
    logger::info(format!("Well Name '{}'", args.wellname.trim()).as_str());
    logger::info(format!("MongoDb Connection '{}'", args.connectionstring).as_str());
    logger::info(format!("Active MQ URL '{}'", args.activemqurl).as_str());
    logger::info(format!("MQ name is '{}'", args.activemqname).as_str());
}
