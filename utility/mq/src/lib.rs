pub fn get_version() -> &'static str {
    "1.0.0"
}

pub fn init() {
    config::create_db_config_if_not_exist();
}

#[cfg(test)]
mod tests {
    use crate::get_version;

    #[test]
    fn version() {
        assert_eq!(get_version(), "1.0.0")
    }
}
