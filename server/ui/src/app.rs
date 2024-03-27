use eframe;
use eframe::{egui, Frame};
use egui::Context;

pub struct App {
    name: String,
    age: u32,
}

impl Default for App {
    fn default() -> Self {
        logger::info("going to init App");
        Self {
            name: String::from(""),
            age: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Program");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
