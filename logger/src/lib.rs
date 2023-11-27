use log::{info, error};

pub fn get_version() -> String {
    String::from("1.0.0")
}

pub fn init() {
    config::init_config_yaml();

    let config_path = common::file_helper::log4rs_yaml_path();
    log4rs::init_file(config_path, Default::default()).unwrap();
}

pub fn info(msg: &str) {
    info!("{}",msg);
}

pub fn error(msg: &str) {
    error!("{}",msg);
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
