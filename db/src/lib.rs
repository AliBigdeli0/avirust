use config::create_db_config_if_not_exist;

pub fn get_db_version() -> &'static str {
    "1.0.0"
}

pub fn init() {
    create_db_config_if_not_exist();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_version() {
        let version = get_db_version();
        assert_eq!(version, "1.0.0");
    }
}
