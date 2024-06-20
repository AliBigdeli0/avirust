use eframe::Error as EframeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("error in eframe: {0}")]
    EfError(#[from] EframeError),
}
