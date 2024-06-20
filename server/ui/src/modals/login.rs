use std::rc::Rc;

use common::{
    constants::EMPTY_STR,
    string_helper::{margin_chars, UIStrMargin},
};
use eframe::egui::{self};
use entity::model::ui_config::UICommonMemory;

use crate::modals::base::BaseModal;

use super::msg::MsgBox;
pub struct LoginModal {
    pub base: BaseModal,
    username: String,
    password: String,
    apply_str: String,
    password_str: String,
    error_message: String,
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
            error_message: EMPTY_STR.to_string(),
            memory,
        }
    }

    pub fn apply(&mut self, dialog_handler: &mut bool) {
        println!("Applied username: {}", self.username);
        println!("Applied password: {}", self.password);

        self.memory.change_address("10.10.10.103:9004".to_string());
        self.error_message = "error in authentication".to_owned();
        *dialog_handler = true
    }

    pub fn update(&mut self, ui: &mut egui::Ui, dialog_handler: &mut bool) {
        if *dialog_handler {
            if self.error_message.trim() != EMPTY_STR {
                MsgBox::error(ui, &mut self.error_message);
            } else {
                self.show_modal(ui, dialog_handler)
            }
            self.base.grayout_back(ui.ctx());
        }
    }

    fn show_modal(&mut self, ui: &mut egui::Ui, dialog_handler: &mut bool) {
        let modal_size: (f32, f32) = (350.0, 400.0);
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
                    let password_textedit =
                        egui::TextEdit::singleline(&mut self.password).password(true);
                    ui.label(self.password_str.as_str());
                    ui.add(password_textedit);
                });

                ui.horizontal(|ui| {
                    ui.add_space(modal_size.0 - 110.0);
                    if ui.button(self.apply_str.as_str()).clicked() {
                        self.apply(dialog_handler);
                    }
                });

                //press enter
                ui.ctx().input(|eui| {
                    if eui.key_pressed(egui::Key::Enter) {
                        self.apply(dialog_handler)
                    }
                });
            });
    }
}
