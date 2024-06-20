use eframe;
use eframe::{egui, Frame};
use egui::Context;
use entity::model::ui_config::UIConfig;

use crate::modals::login::LoginModal;

pub struct App {
    login_modal: LoginModal,
    open_login_at_startup: bool,
    config: UIConfig,
}

impl Default for App {
    fn default() -> Self {
        logger::info("going to init App");
        let uiconfig = config::read_ui_config().unwrap();

        let ret = Self {
            login_modal: LoginModal::new(),
            open_login_at_startup: true,
            config: uiconfig,
        };

        logger::info(
            format!(
                "server address is :  {}:{}",
                ret.config.get_ip().unwrap(),
                ret.config.get_port().unwrap(),
            )
            .as_str(),
        );
        ret
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.open_login_at_startup {
                self.login_modal.update(ui, &mut self.open_login_at_startup);
            } else {
                if ui.button("clicked on me").clicked() {
                    logger::info("button clicked");
                }
            }
        });
    }
}
