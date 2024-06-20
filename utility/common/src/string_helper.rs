//enums
pub enum UIStrMargin {
    Left,
    Right,
    Both,
}

//
pub fn margin_chars(msg: &str, len: usize, mode: UIStrMargin) -> String {
    let marging = String::from(" ").repeat(len);
    match mode {
        UIStrMargin::Left => format!("{}{}", marging, msg),
        UIStrMargin::Right => format!("{}{}", msg, marging),
        UIStrMargin::Both => format!("{}{}{}", marging, msg, marging),
    }
}

#[cfg(test)]
mod test_string_helper {
    use super::*;

    #[test]
    fn test_margin_character() {
        let original = "ali";

        assert_eq!(margin_chars(original, 3, UIStrMargin::Left), "   ali");
        assert_eq!(margin_chars(original, 3, UIStrMargin::Right), "ali   ");
        assert_eq!(margin_chars(original, 3, UIStrMargin::Both), "   ali   ");
    }
}
