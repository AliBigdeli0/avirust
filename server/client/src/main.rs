#![deny(warnings)]

extern crate common;
extern crate config;
extern crate db;
extern crate entity;
extern crate logger;
extern crate web;

#[tokio::main]
async fn main() {
    common::file_helper::change_working_directory();
    logger::init_with_name("client_logs.log");

    print_info();

    web::init();
    web::run_wait(entity::model::auth::StaticAuthentication::new()).await;
}

fn print_info() {
    logger::info("===================================");
    logger::info("going to startup the program...");

    // libs
    logger::info(&format!("entity version is: {}", entity::get_version()));
    logger::info(&format!("common version is: {}", common::get_lib_version()));
    logger::info(&format!("config version is: {}", config::get_version()));
    logger::info(&format!("logger version is: {}", logger::get_version()));
    logger::info(&format!("web version is: {}", web::get_version()));

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

#[cfg(test)]
mod test {
    use entity::dto::Point;
    use entity::serializer_helper::serialize_json;

    #[test]
    fn serialize_out_of_scope() {
        let obj = serialize_json(&Point { x: 10, y: 20 }).unwrap();
        assert!(obj.contains("10"));
        assert!(obj.contains("20"));
    }
}
