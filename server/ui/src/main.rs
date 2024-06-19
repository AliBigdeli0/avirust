#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate logger;

mod app;
mod uierrors;

use common::constants::UI_APP_NAME;
use eframe::egui;
use eframe::egui::{Style, Visuals};
use uierrors::UiError;

pub fn get_ui_version() -> &'static str {
    "1.0.0"
}

fn main() -> Result<(), UiError> {
    logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximize_button(true)
            .with_maximized(true)
            .with_fullsize_content_view(true),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        UI_APP_NAME,
        options,
        Box::new(|creation_context| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            creation_context.egui_ctx.set_style(style);
            Box::<app::App>::default()
        }),
    )
    .map_err(|_| UiError::new("error in run native code".to_owned()))?;
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
