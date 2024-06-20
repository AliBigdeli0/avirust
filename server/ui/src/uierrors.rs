use eframe::Error as EframeError;
use entity::model::ui_config::UiConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("error in eframe: {0}")]
    EfError(#[from] EframeError),

    #[error("error in uiconfig: {0}")]
    UiConfig(#[from] UiConfigError),
}
