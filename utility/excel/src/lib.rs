#![deny(warnings)]

pub fn get_excel_version() -> String {
    String::from("1.0.0")
}

pub mod excel_helper;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        let version = get_excel_version();
        assert_eq!(&version, "1.0.0")
    }
}
