const CONFIG_FILE_CONTENT: &str = include_str!("../resources/log4rs.yml");

use std::{
    fs::File,
    io::Write
};

pub fn get_version() -> &'static str {
    "1.0.0"
}

pub fn init_config_yaml() {
    let config_path = common::file_helper::log4rs_yaml_path();
    common::file_helper::create_dir_if_not_exist(config_path.clone().to_str().unwrap()).unwrap();
    let mut file = File::create(config_path.clone().to_str().unwrap()).unwrap();
    file.write(CONFIG_FILE_CONTENT.as_bytes()).unwrap();
    let _ = file.flush().unwrap();
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
