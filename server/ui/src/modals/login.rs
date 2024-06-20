use std::rc::Rc;

use common::string_helper::{margin_chars, UIStrMargin};
use eframe::egui;
use entity::model::ui_config::UICommonMemory;

use crate::modals::base::BaseModal;
pub struct LoginModal {
    pub base: BaseModal,
    username: String,
    password: String,
    apply_str: String,
    password_str: String,
    memory: Rc<UICommonMemory>,
}

impl LoginModal {
    pub fn new(memory: Rc<UICommonMemory>) -> Self {
        LoginModal {
            base: BaseModal::new(),
            username: "".to_owned(),
            password: "".to_owned(),
            apply_str: margin_chars("Apply", 10, UIStrMargin::Both),
            password_str: margin_chars("Password:", 1, UIStrMargin::Right),
            memory,
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui, dialog_handler: &mut bool) {
        if *dialog_handler {
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
                        ui.label(self.password_str.as_str());
                        ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                    });

                    ui.horizontal(|ui| {
                        ui.add_space(modal_size.0 - 110.0);
                        if ui.button(self.apply_str.as_str()).clicked() {
                            println!("Applied username: {}", self.username);
                            println!("Applied password: {}", self.password);

                            self.memory.change_address("10.10.10.103:9004".to_string());
                            *dialog_handler = false
                        }
                    });
                });

            self.base.grayout_back(ui.ctx());
        }
    }
}
