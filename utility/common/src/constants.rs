pub const UI_APP_NAME: &str = "MED Well View";

#[cfg(test)]
mod test_constatns {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(UI_APP_NAME, "MED Well View");
    }
}
