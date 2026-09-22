pub fn get_version() -> &'static str {
    "2.0.0"
}

pub mod mq;
pub mod mqerror;

#[cfg(test)]
mod tests {
    use crate::get_version;

    #[test]
    fn version() {
        assert_eq!(get_version(), "2.0.0")
    }
}
