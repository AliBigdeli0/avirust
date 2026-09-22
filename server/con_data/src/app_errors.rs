use db::dberror;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] dberror::DbError),

    #[error("MQ error: {0}")]
    MQ(#[from] mq::mqerror::MqError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Text(String),
}

pub type AppResult<T> = std::result::Result<T, AppError>;
