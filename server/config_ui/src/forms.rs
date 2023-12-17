
pub trait BaseForms {
    fn show_controls(&mut self,_ctx: &egui::Context, _frame: &mut eframe::Frame);
    fn menu_name(&mut self) ->  Box<&str>;
}


// ------------------------------------------
#[derive(Default, Clone)]
#[cfg_attr(feature="serde", derive(serde::Deserialize, serde::Serialize))]
pub struct HomeForm {

}


impl BaseForms for HomeForm {
    fn show_controls(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
         egui::CentralPanel::default()
            .show(ctx, |ui|{
                ui.vertical_centered(|ui|{
                    ui.heading(self.menu_name().to_string())
                });
                ui.separator();
            });
    }

    fn menu_name(&mut self) ->  Box<&str> {
        Box::new("☫ Home")
    }
}


// ------------------------------------------
#[derive(Default)]
#[cfg_attr(feature="serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WellMangement {

}

impl BaseForms for WellMangement {
    fn show_controls(&mut self,ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .show(ctx, |ui|{
                ui.vertical_centered(|ui|{
                    ui.heading(self.menu_name().to_string())
                });
                ui.separator();
            });
    }

    fn menu_name(&mut self) ->  Box<&str> {
        Box::new("⛃ Well Management")
    }
}