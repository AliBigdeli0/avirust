#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(warnings)]
#![deny(dead_code)]

extern crate logger;

mod eguiui;
mod slintui;
mod uierrors;

use config::create_ui_config_if_not_exist;
use uierrors::UiError;

pub fn get_ui_version() -> &'static str {
    "1.0.0"
}

fn main() -> Result<(), UiError> {
    logger::init();

    create_ui_config_if_not_exist().map_err(|e| UiError::UiEguiConfig(e))?;

    slintui::run_slive_gui()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_version() {
        let version = get_ui_version();
        assert_eq!(version, "1.0.0");
    }
}
