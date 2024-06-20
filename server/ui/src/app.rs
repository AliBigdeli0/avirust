use eframe;
use eframe::{egui, Frame};
use egui::Context;

use crate::modals::login::LoginModal;

pub struct App {
    login_modal: LoginModal,
    open_login_at_startup: bool,
}

impl Default for App {
    fn default() -> Self {
        logger::info("going to init App");
        Self {
            login_modal: LoginModal::new(),
            open_login_at_startup: true,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("clicked on me").clicked() {
                logger::info("button clicked");
            }

            if self.open_login_at_startup {
                self.login_modal.update(ui, &mut self.open_login_at_startup);
            }
        });
    }
}
