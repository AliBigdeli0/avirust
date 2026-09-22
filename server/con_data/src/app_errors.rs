use db::dberror;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] dberror::DbError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type AppResult<T> = std::result::Result<T, AppError>;
