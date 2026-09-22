#![deny(warnings)]

extern crate log;

use log::{debug, error, info, warn};

pub fn get_version() -> String {
    String::from("1.0.0")
}

pub fn init() {
    config::init_config_yaml(None);

    let config_path = common::file_helper::log4rs_yaml_path(None);
    log4rs::init_file(config_path, Default::default()).unwrap();
}

pub fn init_with_name(file_name: &str) {
    config::init_config_yaml(Some(file_name));

    let config_path = common::file_helper::log4rs_yaml_path(Some(file_name));
    log4rs::init_file(config_path, Default::default()).unwrap();
}

pub fn info(msg: &str) {
    info!("{}", msg);
}

pub fn warn(msg: &str) {
    warn!("{}", msg);
}

pub fn error(msg: &str) {
    error!("{}", msg);
}

pub fn debug(msg: &str) {
    debug!("{}", msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        let ver = get_version();
        assert_eq!(&ver, "1.0.0");
    }
}
