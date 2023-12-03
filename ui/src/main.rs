#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use eframe::egui;

pub fn get_ui_version() -> &'static str {
    "1.0.0"
}

fn main() -> Result<(), eframe::Error> {
    logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MED UI",
        options,
        Box::new(|_| {
            Box::<app::App>::default()
        }),
    )
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