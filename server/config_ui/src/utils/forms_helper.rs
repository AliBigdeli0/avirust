use egui::{Context, Label, Spinner, Widget};
use egui_modal::Modal;

enum ModalProperties {
    YesNo(egui_modal::Icon),
    Ok(egui_modal::Icon),
    Spinner(),
}

pub enum ModalResult {
    None,
    Ok,
    No,
    Yes,
}

fn show_base(
    ctx: &Context,
    message: &str,
    title: &str,
    open: &mut bool,
    prop: ModalProperties,
) -> ModalResult {
    let modal = Modal::new(ctx, "show_info_modal");
    let mut result: ModalResult = ModalResult::None;
    modal.show(|ui| match prop {
        ModalProperties::Ok(icon) => {
            modal.title(ui, title);
            modal.frame(ui, |ui| {
                modal.body_and_icon(ui, message, icon);
            });

            modal.buttons(ui, |ui| {
                let ok_button = modal.button(ui, "  ✔Ok  ");
                if ok_button.clicked() {
                    *open = false;
                    result = ModalResult::Ok;
                }
            });
        }
        ModalProperties::YesNo(yes_no_icon) => {
            modal.title(ui, title);
            modal.frame(ui, |ui| {
                modal.body_and_icon(ui, message, yes_no_icon);
            });

            modal.buttons(ui, |ui| {
                let yes_button = modal.button(ui, "  ✔Yes  ");
                if yes_button.clicked() {
                    *open = false;
                    result = ModalResult::Yes;
                }

                let no_button = modal.button(ui, "  ✖No  ");
                if no_button.clicked() {
                    *open = false;
                    result = ModalResult::No;
                }
            });
        }
        ModalProperties::Spinner() => modal.frame(ui, |frame_ui| {
            Spinner::new().ui(frame_ui);
            frame_ui.add(Label::new(message))
        }),
    });
    if *open {
        modal.open();
    }
    result
}

pub fn show_info(ctx: &Context, message: &str, title: &str, open: &mut bool) -> ModalResult {
    let properties = ModalProperties::Ok(egui_modal::Icon::Info);
    show_base(ctx, message, title, open, properties)
}

pub fn show_yes_no(ctx: &Context, message: &str, title: &str, open: &mut bool) -> ModalResult {
    let properties = ModalProperties::YesNo(egui_modal::Icon::Info);
    show_base(ctx, message, title, open, properties)
}

pub fn show_spinner(ctx: &Context, message: &str, open: &mut bool) -> ModalResult {
    let prop = ModalProperties::Spinner();
    show_base(ctx, message, "", open, prop)
}
