use std::sync::Arc;

use common::env_helper::EnvHelper;

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
    logger::info("Con Data has started");
    logger::info(&format!("Well ID is :{}", &well_id));
    logger::info(&format!("DB connection:'{}'", &mongo_address));
    logger::info(&format!("Redis connection:'{}'", &redis_address));
    logger::info(&format!("AMQ connection:'{}'", &activemq_address));

    print_info();
}

fn print_info() {
    // libs
    logger::info(&format!("entity version is: {}", entity::get_version()));
    logger::info(&format!("common version is: {}", common::get_lib_version()));
    logger::info(&format!("config version is: {}", config::get_version()));
    logger::info(&format!("logger version is: {}", logger::get_version()));

    // directory ifo
    logger::info(&format!(
        "working directory is: {}",
        common::file_helper::current_path().to_str().unwrap()
    ));
    logger::info(&format!(
        "configure directory is: {}",
        common::file_helper::config_path().to_str().unwrap()
    ));
}
