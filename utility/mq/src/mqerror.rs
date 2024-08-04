#[derive(Debug)]
pub enum MqError {
    TextErr(String),
}

pub type Result<T> = std::result::Result<T, MqError>;
