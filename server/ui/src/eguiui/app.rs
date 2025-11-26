use std::rc::Rc;

use eframe;
use eframe::{egui, Frame};
use egui::Context;
use entity::model::ui_config::{SimpleAuthenticator, UICommonMemory};

use crate::eguiui::modals::login::LoginModal;

#[allow(dead_code)]

pub struct App {
    login_modal: LoginModal<SimpleAuthenticator>,
    open_login_at_startup: bool,
    memory: Rc<UICommonMemory>,
}

impl Default for App {
    fn default() -> Self {
        logger::info("going to init App");
        let uiconfig = config::read_ui_config().unwrap();
        let common_memory = Rc::new(UICommonMemory::new(uiconfig));
        let authenticator = Box::new(SimpleAuthenticator);
        let ret = Self {
            login_modal: LoginModal::new(common_memory.clone(), authenticator),
            open_login_at_startup: true,
            memory: common_memory,
        };

        logger::info(
            format!(
                "server address is :  {}:{}",
                ret.memory.config.borrow().get_ip().unwrap(),
                ret.memory.config.borrow().get_port().unwrap(),
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
                    let ip = self.memory.config.borrow().get_ip().unwrap();
                    logger::info(format!("the ip is: {}", ip).as_str());
                    logger::info(
                        format!("verbose is: {}", self.memory.config.borrow().get_verbose())
                            .as_str(),
                    );
                }
            }
        });
    }
}
