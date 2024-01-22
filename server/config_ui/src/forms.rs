use egui::{Grid, Window};

use crate::utils::forms_helper;

pub trait BaseForms {
    fn show_controls(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame);
    fn menu_name(&mut self) -> Box<&str>;
}

// ------------------------------------------
#[derive(Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct HomeForm {
    open_modal: bool,
    open_message: bool,
}

impl BaseForms for HomeForm {
    fn show_controls(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading(self.menu_name().to_string()));
            ui.separator();

            let min_size = (ctx.screen_rect().width() / 2.0) - 15.0;
            Grid::new("some data")
                .min_col_width(min_size)
                .striped(true)
                .show(ui, |grid_ui| {
                    grid_ui.label("one");
                    grid_ui.label("two");
                    grid_ui.end_row();

                    if grid_ui.button("open modal here").clicked() {
                        self.open_modal = true
                    }

                    if grid_ui.button("the modal").clicked() {
                        self.open_message = true
                    }
                    grid_ui.end_row();

                    let _ = grid_ui.button("here three");
                    let _ = grid_ui.button("here four");
                    grid_ui.end_row();

                    let _ = grid_ui.button("here five");
                    let _ = grid_ui.button("here six");
                    grid_ui.end_row();
                });

            let mut close = false;
            Window::new("my own windows")
                .open(&mut self.open_modal)
                .min_width(400.0)
                .min_height(300.0)
                .collapsible(false)
                .show(ctx, |win_ui| {
                    if win_ui.button("Apply").clicked() {
                        close = true
                    }
                });
            self.open_modal &= !close;

            if self.open_message {
                forms_helper::show_info(ctx, "The data has been saved", "warning", &mut self.open_message);
            }
        });
    }

    fn menu_name(&mut self) -> Box<&str> {
        Box::new("☫ Home")
    }
}

// ------------------------------------------
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WellMangement {}

impl BaseForms for WellMangement {
    fn show_controls(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading(self.menu_name().to_string()));
            ui.separator();
        });
    }

    fn menu_name(&mut self) -> Box<&str> {
        Box::new("⛃ Well Management")
    }
}
