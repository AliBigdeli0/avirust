// enum ModalProperties {
//     // YesNo(egui_modal::Icon),
//     // Ok(egui_modal::Icon),
//     // Spinner(),
// }

pub enum ModalResult {
    None,
    // Ok,
    // No,
    // Yes,
}

fn show_base(// ctx: &egui::Context,
    // message: &str,
    // title: &str,
    // open: &mut bool,
    // prop: ModalProperties,
) -> ModalResult {
    //let modal = egui_modal::Modal::new(ctx, "show_info_modal");
    // let modal = egui_modal::Modal::new(ctx)
    // let modal = egui_modal::Modal::new(&ctx.clone(), "show_info_modal");
    // let mut result: ModalResult = ModalResult::None;
    // modal.show(|ui| match prop {
    //     ModalProperties::Ok(icon) => {
    //         modal.title(ui, title);
    //         modal.frame(ui, |ui| {
    //             modal.body_and_icon(ui, message, icon);
    //         });

    //         modal.buttons(ui, |ui| {
    //             let ok_button = modal.button(ui, "  ✔Ok  ");
    //             if ok_button.clicked() {
    //                 *open = false;
    //                 result = ModalResult::Ok;
    //             }
    //         });
    //     }
    //     ModalProperties::YesNo(yes_no_icon) => {
    //         modal.title(ui, title);
    //         modal.frame(ui, |ui| {
    //             modal.body_and_icon(ui, message, yes_no_icon);
    //         });

    //         modal.buttons(ui, |ui| {
    //             let yes_button = modal.button(ui, "  ✔Yes  ");
    //             if yes_button.clicked() {
    //                 *open = false;
    //                 result = ModalResult::Yes;
    //             }

    //             let no_button = modal.button(ui, "  ✖No  ");
    //             if no_button.clicked() {
    //                 *open = false;
    //                 result = ModalResult::No;
    //             }
    //         });
    //     }
    //     ModalProperties::Spinner() => modal.frame(ui, |frame_ui| {
    //         egui::Spinner::new().ui(frame_ui);
    //         let text = egui::WidgetText::from(message);
    //         frame_ui.add(Label::new(text))
    //     }),
    // });
    // if *open {
    //     modal.open();
    // }
    ModalResult::None
}

pub fn show_info() -> ModalResult {
    // let properties = ModalProperties::Ok(egui_modal::Icon::Info);
    show_base()
}

pub fn show_yes_no() -> ModalResult {
    // let properties = ModalProperties::YesNo(egui_modal::Icon::Info);
    show_base()
}

pub fn show_spinner() -> ModalResult {
    // let prop = ModalProperties::Spinner();
    show_base()
}
