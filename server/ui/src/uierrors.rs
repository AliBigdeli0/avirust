use eframe::Error as EframeError;
use entity::model::ui_config::UiConfigError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("error in eframe: {0}")]
    EfError(#[from] EframeError),

    #[error("error in egui config: {0}")]
    UiEguiConfig(#[from] UiConfigError),

    #[error("error in slint : {0}")]
    SlintError(String),
}

//result
pub type Result<T> = std::result::Result<T, UiError>;
