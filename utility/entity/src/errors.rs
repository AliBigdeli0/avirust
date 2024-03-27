pub trait ErrorTo<T> {
    fn to(&self) -> &T;
}

#[derive(Debug)]
pub enum ExcelError {
    Message(String),
}

impl ErrorTo<String> for ExcelError {
    fn to(&self) -> &String {
        match self {
            Self::Message(ret_msg) => ret_msg,
        }
    }
}

#[cfg(test)]
mod test_errors {
    use super::*;

    enum MockError {
        Message(String),
    }

    impl ErrorTo<String> for MockError {
        fn to(&self) -> &String {
            match self {
                Self::Message(ret_msg) => ret_msg,
            }
        }
    }

    fn mock_error(i: u8) -> Result<impl AsRef<str>, MockError> {
        if i < 8 {
            return Err(MockError::Message("error".to_string()));
        }
        Ok("hello")
    }

    #[test]
    fn test_base_error_message() {
        let mock_msg = match mock_error(7) {
            Ok(msg) => msg.as_ref().to_string(),
            Err(err) => err.to().to_owned(),
        };

        assert_eq!(mock_msg, "error");
    }
}
