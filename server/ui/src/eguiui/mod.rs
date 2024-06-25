use crate::uierrors::UiError;

pub mod app;
pub mod modals;

use common::constants::UI_APP_NAME;

use eframe::egui::{self};
use eframe::egui::{Style, Visuals};

pub fn _run_egui_ui() -> Result<(), UiError> {
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
