pub fn get_lib_version() -> &'static str {
    "1.0.0"
}

pub mod file_helper;
pub mod time_helper;
pub mod hash_helper;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_version() {
        assert_eq!(get_lib_version(), "1.0.0");
    }
}
