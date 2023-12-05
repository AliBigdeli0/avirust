const CONFIG_FILE_CONTENT: &str = include_str!("../resources/log4rs.yml");

use std::{
    fs::File,
    io::Write,
};
use std::io::Read;
use entity::model::server_config::ServerConfig;
use entity::model::db_config::DBConfig;

pub fn get_version() -> &'static str {
    "1.0.0"
}

pub fn init_config_yaml() {
    let config_path = common::file_helper::log4rs_yaml_path();

    if !common::file_helper::file_exist(config_path.clone()) {
        let mut file = File::create(config_path.clone().to_str().unwrap()).unwrap();
        file.write(CONFIG_FILE_CONTENT.as_bytes()).unwrap();
        let _ = file.flush().unwrap();
    }
}

pub fn create_server_config_if_not_exist() {
    let server_config_path = common::file_helper::server_yaml_path();
    let object_to_serliaze: ServerConfig = Default::default();

    entity::serializer_helper::write_in_file_if_not_exist(server_config_path, &object_to_serliaze).unwrap();
}

pub fn create_db_config_if_not_exist() {
    let db_config_path = common::file_helper::db_yaml_path();
    let db_to_serialize: DBConfig = Default::default();
    entity::serializer_helper::write_in_file_if_not_exist(db_config_path, &db_to_serialize).unwrap();
}

pub fn read_server_config() -> Result<ServerConfig, std::io::Error> {
    let server_config_path = common::file_helper::server_yaml_path();

    let mut file = File::open(server_config_path.clone()).unwrap();

    let mut server_config_yaml_string: String = String::from("");
    let _ = file.read_to_string(&mut server_config_yaml_string).unwrap();

    let server_config: ServerConfig = entity::serializer_helper::deserilize_yaml(&mut server_config_yaml_string).unwrap();
    Ok(server_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        assert_eq!(get_version(), "1.0.0");
    }

    #[test]
    fn config_file_content_test() {
        assert_ne!(CONFIG_FILE_CONTENT, "");
    }
}
