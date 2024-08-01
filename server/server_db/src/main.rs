#![deny(warnings)]

use common::file_helper::CONFIG_EXTENSION;

extern crate common;
extern crate logger;

#[tokio::main]
async fn main() {
    common::file_helper::change_working_directory();

    logger::init_with_name(format!("mehran{}", CONFIG_EXTENSION).as_str());

    logger::info("server started");

    logger::error("server exited!");
}
