use egui::{Button, Ui, Visuals};

use crate::{
    forms::{BaseForms, HomeForm, WellMangement},
    utils,
};

#[derive(Clone, Debug)]
enum FormTypes {
    Home,
    WellManagement,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppTemplate {
    #[serde(skip)]
    home_form: Box<dyn BaseForms>,

    #[serde(skip)]
    well_management_form: Box<dyn BaseForms>,

    #[serde(skip)]
    state: FormTypes,

    #[serde(skip)]
    style: Visuals,

    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    font_scale: f32,
}

impl Default for AppTemplate {
    fn default() -> Self {
        Self {
            value: 0.1,
            style: Visuals::dark(),
            state: FormTypes::Home,
            home_form: Box::new(HomeForm::default()),
            well_management_form: Box::new(WellMangement::default()),
            font_scale: 1.4,
        }
    }
}

impl AppTemplate {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    fn forms_iter(&mut self) -> impl Iterator<Item = FormTypes> {
        let vec: Vec<FormTypes> = vec![FormTypes::Home, FormTypes::WellManagement];

        vec.into_iter()
    }

    fn add_banner(&mut self, ui: &mut Ui) {
        ui.add(egui::widgets::Label::new("Wells Configurations"));
        ui.separator();
    }

    fn add_visual_styles(&mut self, ui: &mut Ui) {
        if self.style.dark_mode {
            if ui.add(Button::new("☀").frame(false)).clicked() {
                self.style = Visuals::light();
            }
        } else {
            if ui.add(Button::new("🌙").frame(false)).clicked() {
                self.style = Visuals::dark();
            }
        }
        ui.separator();
    }

    fn add_all_menus(&mut self, ui: &mut Ui) {
        for form_type in self.forms_iter() {
            let form = self.get_form_with_type(&mut form_type.clone());

            if ui.button(&form.menu_name().to_string()).clicked() {
                self.state = form_type;
            }

            ui.separator();
        }
    }

    fn get_form_with_type(&mut self, form_type: &mut FormTypes) -> &mut dyn BaseForms {
        match form_type {
            FormTypes::Home => self.home_form.as_mut(),
            FormTypes::WellManagement => self.well_management_form.as_mut(),
        }
    }
}

impl eframe::App for AppTemplate {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        utils::styles_helper::set_font_scale(ctx, self.font_scale);

        ctx.set_visuals(self.style.clone());
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.add_visual_styles(ui);
                self.add_banner(ui);
                self.add_all_menus(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            let form = self.get_form_with_type(&mut self.state.clone());
            form.show_controls(ctx, frame);
        });
    }
}
