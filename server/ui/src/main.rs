#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(warnings)]

extern crate logger;

mod app;
mod modals;
mod uierrors;

use common::constants::UI_APP_NAME;
use config::create_ui_config_if_not_exist;
use eframe::egui::{self};
use eframe::egui::{Style, Visuals};
use uierrors::UiError;

pub fn get_ui_version() -> &'static str {
    "1.0.0"
}

fn main() -> Result<(), UiError> {
    logger::init();

    create_ui_config_if_not_exist().map_err(|e| UiError::UiConfig(e))?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullsize_content_view(true)
            .with_close_button(true)
            .with_resizable(true)
            .with_title(UI_APP_NAME)
            .with_inner_size(egui::Vec2 {
                x: 1400.0,
                y: 900.0,
            }),
        centered: true,
        window_builder: Some(Box::new(|wb| {
            wb.with_fullsize_content_view(true)
                .with_maximized(true)
                .with_maximize_button(false)
                .with_minimize_button(false)
        })),
        event_loop_builder: Some(Box::new(|_eb| {})),
        ..Default::default()
    };

    eframe::run_native(
        "",
        options,
        Box::new(|creation_context| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            creation_context.egui_ctx.set_style(style);
            Box::<app::App>::default()
        }),
    )?;
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
