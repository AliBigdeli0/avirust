use std::fmt;

// -----------------------------------------
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

// -----------------------------------------

#[derive(Debug)]
pub struct MQErrors;

impl fmt::Display for MQErrors {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("invalid mq error")
    }
}

impl error_stack::Context for MQErrors {}
// -----------------------------------------

#[cfg(test)]
mod test_errors {
    use error_stack::ResultExt;

    use super::*;
    use std::fs::File;

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

    #[test]
    fn mq_error() {
        let file_exchange = File::open("NOT_EXIST_FILE.txt")
            .attach_printable_lazy(|| "error in read test file")
            .change_context(MQErrors);

        match file_exchange {
            Ok(_) => assert!(false, "file can not be created, because it is fake"),
            Err(_) => assert!(true, "everyhing is OK"),
        };
    }
}
