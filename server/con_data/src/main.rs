use std::{sync::Arc, time::Duration};
use tokio::{sync::mpsc, time::sleep};
use tokio_util::sync::CancellationToken;

// internal
use crate::{app_errors::AppResult, receiver::start_activemq_reader, sender::start_redis_writer};
use common::{constants::REASTART_APP_SEC, env_helper::EnvHelper};

mod app_errors;
mod receiver;
mod sender;
mod well_actions;

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
    let cancel = CancellationToken::new();

    let _ = well_actions::read_well(mongo_address, well_id).await?;

    let (tx, rx) = mpsc::channel::<String>(200_000);

    let mq_handle = start_activemq_reader(tx, cancel.clone());

    let memory_handle = start_redis_writer(rx);

    cancel.cancel();

    mq_handle.await??;

    memory_handle.await??;

    Ok(())
}
