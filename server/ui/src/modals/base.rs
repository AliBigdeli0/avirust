use eframe::egui;

pub struct BaseModal {}

impl BaseModal {
    pub fn new() -> Self {
        BaseModal {}
    }

    pub fn center_position(&self, ctx: &egui::Context, modal_size: (f32, f32)) -> egui::Pos2 {
        let screen_rect = ctx.screen_rect();
        let center = screen_rect.center();
        let x = center.x - (modal_size.0 / 2.0);
        let y = center.y;
        egui::Pos2::new(x, y)
    }

    pub fn grayout_back(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| ui.set_enabled(true));
    }
}
