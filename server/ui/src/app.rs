use eframe;
use eframe::{egui, Frame};
use egui::Context;

pub struct App {}

impl Default for App {
    fn default() -> Self {
        logger::info("going to init App");
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}
