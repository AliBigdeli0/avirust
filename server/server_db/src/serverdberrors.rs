#![allow(dead_code)]
#[derive(Debug)]
pub enum ServerDbError {
    TextErr(String),
}

pub type Result<T> = std::result::Result<T, ServerDbError>;
