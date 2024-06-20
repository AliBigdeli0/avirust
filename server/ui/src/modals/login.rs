use eframe::egui::{self};

use crate::modals::base::BaseModal;
pub struct LoginModal {
    pub base: BaseModal,
    username: String,
    password: String,
}

impl LoginModal {
    pub fn new() -> Self {
        LoginModal {
            base: BaseModal::new(),
            username: "".to_owned(),
            password: "".to_owned(),
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui, is_open: &mut bool) {
        if *is_open {
            let modal_size: (f32, f32) = (300.0, 400.0);
            let screen_center = self.base.center_position(ui.ctx(), modal_size);

            egui::Window::new("Login")
                .resizable(false)
                .default_size(modal_size)
                .fixed_pos(screen_center)
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Username:");
                        ui.add(egui::TextEdit::singleline(&mut self.username));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Password: ");
                        ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                    });

                    ui.horizontal(|ui| {
                        ui.add_space(modal_size.0 - 65.0);
                        if ui.button("   Apply   ").clicked() {
                            println!("Applied username: {}", self.username);
                            println!("Applied password: {}", self.password);
                        }
                    });
                });

            self.base.grayout_back(ui.ctx());
        }
    }
}
