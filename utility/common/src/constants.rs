pub const UI_APP_NAME: &str = "MED Well View";
pub const EMPTY_STR: &str = "";
#[cfg(test)]
mod test_constatns {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(UI_APP_NAME, "MED Well View");
    }
}
