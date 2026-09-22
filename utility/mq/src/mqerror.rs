use iridium_stomp::ConnError;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, thiserror::Error)]
pub enum MqError {
    #[error("{0}")]
    TextErr(String),

    #[error("Url parse Error: {0}")]
    ParserError(#[from] url::ParseError),

    #[error("Stomp Connection Error: {0}")]
    StompConnError(#[from] ConnError),

    #[error("Send Channel Error: {0}")]
    SendError(#[from] SendError<String>),
}

pub type MQResult<T> = std::result::Result<T, MqError>;
