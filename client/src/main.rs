use entity::model::server_config::ServerConfig;

fn main() {
    logger::init();
    print_info();

    config::create_server_config_if_not_exist();

    let server_config:ServerConfig = config::read_server_config().unwrap();
    logger::info(&format!("{}:{}", server_config.server.listen, server_config.server.port));
}

fn print_info() {
    logger::info("===================================");
    logger::info("going to startup the program...");

    // libs
    logger::info(&format!("entity version is: {}", entity::get_version()));
    logger::info(&format!("common version is: {}", common::get_lib_version()));
    logger::info(&format!("config version is: {}", config::get_version()));
    logger::info(&format!("logger version is: {}", logger::get_version()));

    // directory ifo
    logger::info(&format!("working directory is: {}", common::file_helper::current_path().to_str().unwrap()));
    logger::info(&format!("configure directory is: {}", common::file_helper::config_path().to_str().unwrap()));
}


#[cfg(test)]
mod test {
    use entity::serializer_helper::serialize_json;
    use entity::dto::Point;

    #[test]
    fn serialize_out_of_scope() {
        let obj = serialize_json(&Point { x: 10, y: 20 }).unwrap();
        assert!(obj.contains("10"));
        assert!(obj.contains("20"));
    }
}