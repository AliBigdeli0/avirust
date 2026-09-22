use mongodb::{Client, Database};
use std::sync::Arc;

// internal
use common::env_helper::EnvHelper;
use db::dberror;

mod app_errors;

extern crate common;
extern crate config;

const BANNER: &str = include_str!("../assets/banner.txt");

#[tokio::main]
async fn main() -> app_errors::AppResult<()> {
    common::file_helper::change_working_directory();

    let well_id = Arc::new(EnvHelper::get_well_id());
    let mongo_address = Arc::new(EnvHelper::get_mongo_address());
    let redis_address = Arc::new(EnvHelper::get_redis_address());
    let activemq_address = Arc::new(EnvHelper::get_amq_address());

    logger::init_with_name(&format!("con_data_{}.log", &well_id));

    logger::info(BANNER);
    logger::info("Con Data has started");
    logger::info(&format!("Well ID is :{}", &well_id));
    logger::info(&format!("DB connection:'{}'", &mongo_address));
    logger::info(&format!("Redis connection:'{}'", &redis_address));
    logger::info(&format!("AMQ connection:'{}'", &activemq_address));

    print_info();

    let (_, _) = match init_db(&mongo_address).await {
        Ok((client, db)) => (client, db),
        Err(err) => {
            logger::error(&format!("{err}"));

            return Err(err.into());
        }
    };

    logger::info("Database is prepaerd!");

    Ok(())
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

async fn init_db(connection_str: &Arc<String>) -> dberror::Result<(Client, Database)> {
    logger::info("preparing database...");
    let (client, database) = db::mongo::init_with_connection_string(connection_str).await?;

    Ok((client, database))
}
