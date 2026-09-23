use db::dberror;
use entity::errors::DbModelError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] dberror::DbError),

    #[error("MQ error: {0}")]
    MQ(#[from] mq::mqerror::MqError),

    #[error("Task join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Model Error: {0}")]
    ModelError(#[from] DbModelError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Text(String),
}

pub type AppResult<T> = std::result::Result<T, AppError>;
