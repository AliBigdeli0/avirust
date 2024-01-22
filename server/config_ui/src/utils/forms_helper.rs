use egui::{Context, Button};
use egui_modal::{Modal, Icon, ModalStyle};

pub fn show_info(ctx: &Context, message: &str, title: &str, open: &mut bool) {
    let modal = Modal::new(ctx, "show_info_modal");
    modal.show(|ui| {
        modal.title(ui, title);
        modal.frame(ui, |ui| {
            modal.body_and_icon(ui,message, egui_modal::Icon::Info);
        });

        modal.buttons(ui,|ui| {
            let ok_button = modal.button(ui, "Ok");
            if ok_button.clicked() {
                *open = false;
            }
        });
    });
    if  *open {
        modal.open()
    }
}
