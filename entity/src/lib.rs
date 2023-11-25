pub fn get_version() -> &'static str{
    "1.0.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        let ver = get_version();
        assert_eq!(ver, "1.0.0");
    }
}
