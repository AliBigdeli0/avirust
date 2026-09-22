#[derive(Debug, thiserror::Error)]
pub enum MqError {
    #[error("{0}")]
    TextErr(String),
}

pub type Result<T> = std::result::Result<T, MqError>;
