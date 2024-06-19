use std::fmt;

//
#[derive(Debug)]
pub struct UiError {
    msg: String,
}

impl UiError {
    pub fn new(m: String) -> Self {
        UiError { msg: m }
    }
}

impl fmt::Display for UiError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        logger::error(&self.msg.as_str());
        fmt.write_str(self.msg.as_str())
    }
}

impl error_stack::Context for UiError {}
