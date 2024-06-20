use eframe::egui::{self};

use crate::modals::base::BaseModal;
pub struct LoginModal {
    pub base: BaseModal,
}

impl LoginModal {
    pub fn new() -> Self {
        LoginModal {
            base: BaseModal::new(),
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui, is_open: &mut bool) {
        if *is_open {
            let screen_center = self.base.center_position(ui.ctx());

            egui::Window::new("Login")
                .resizable(false)
                .default_size((300.0, 400.0))
                .fixed_pos(screen_center)
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                    ui.label("This is a modal dialog!");
                    ui.label("This is a modal dialog!");
                    ui.label("This is a modal dialog!");
                    ui.label("This is a modal dialog!");
                    ui.label("This is a modal dialog!");

                    if ui.button("Close").clicked() {
                        *is_open = false;
                    }
                });

            self.base.grayout_back(ui.ctx());
        }
    }
}
