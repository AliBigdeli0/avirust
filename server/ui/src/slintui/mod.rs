use crate::uierrors::UiError;

pub mod forms;

pub fn run_slive_gui() -> Result<(), UiError> {
    forms::login::run_login_window()?;
    Ok(())
}
