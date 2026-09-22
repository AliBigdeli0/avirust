#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("{0}")]
    TextError(String),
}

pub type Result<T> = std::result::Result<T, DbError>;
