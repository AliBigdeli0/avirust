#![deny(warnings)]

extern crate config;
extern crate mongodb;

pub mod mongo;

pub fn get_db_version() -> &'static str {
    "1.0.0"
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
