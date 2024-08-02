#[derive(Debug)]
pub enum DbError {
    TextError(String),
}

pub type Result<T> = std::result::Result<T, DbError>;
