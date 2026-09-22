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

    logger::init_with_name(&format!("con_data_{}.log", &well_id));

    logger::info(BANNER);
    logger::info("Con Data has started");
    logger::info(&format!("Well ID is :{}", &well_id));
    logger::info(&format!("DB connection:'{}'", &mongo_address));
}
