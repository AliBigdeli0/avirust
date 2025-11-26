use common::{
    constants::EMPTY_STR,
    string_helper::{margin_chars, UIStrMargin},
};
use eframe::egui::{self, Label};

use super::base::BaseModal;

#[allow(dead_code)]
pub struct MsgBox;

//
impl MsgBox {
    pub fn _inf(ui: &mut egui::Ui, msg: &mut String) {
        MsgBox::show_box(ui, msg, "❗ Info");
    }

    pub fn error(ui: &mut egui::Ui, msg: &mut String) {
        MsgBox::show_box(ui, msg, "❌ Error");
    }

    fn show_box(ui: &mut egui::Ui, msg: &mut String, title: &str) {
        if msg != EMPTY_STR {
            let modal_size: (f32, f32) = (400.0, 80.0);
            let screen_center = BaseModal::new().center_position(ui.ctx(), modal_size);
            egui::Window::new(title)
                .resizable(false)
                .fixed_pos(screen_center)
                .collapsible(false)
                .movable(true)
                .show(ui.ctx(), move |ui| {
                    ui.add_sized(modal_size, Label::new(msg.to_string()).wrap(true));

                    if ui
                        .button(margin_chars("OK", 10, UIStrMargin::Both))
                        .clicked()
                    {
                        *msg = EMPTY_STR.to_string()
                    }
                });
        }
    }
}
