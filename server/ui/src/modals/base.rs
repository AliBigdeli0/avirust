use eframe::egui;

pub struct BaseModal {}

impl BaseModal {
    pub fn new() -> Self {
        BaseModal {}
    }

    pub fn center_position(&self, ctx: &egui::Context) -> egui::Pos2 {
        let screen_rect = ctx.screen_rect();
        let center = screen_rect.center();
        egui::Pos2::new(center.x, center.y)
    }

    pub fn grayout_back(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| ui.set_enabled(true));
    }
}
