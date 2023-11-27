const CONFIG_FILE_CONTENT: &str = include_str!("../resources/log4rs.yml");

use std::fs::File;
use std::io::Write;
use log::{info, error};

pub fn get_version() -> String {
    String::from("1.0.0")
}

pub fn init() {
    let config_path = common::file_helper::log4rs_yaml_path();
    {
        let mut file = File::create(config_path.clone()).unwrap();
        file.write(CONFIG_FILE_CONTENT.as_bytes()).unwrap();
        file.flush().unwrap();
    }
    log4rs::init_file(config_path.clone(), Default::default()).unwrap();
}

pub fn info(msg: &'static str) {
    info!("{}",msg);
}

pub fn error(msg: &'static str) {
    error!("{}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        let ver = get_version();
        assert_eq!(&ver, "1.0.0");
    }

    #[test]
    fn config_file_content_test() {
        assert_ne!(CONFIG_FILE_CONTENT, "");
    }
}
