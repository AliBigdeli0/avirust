

pub fn set_font_scale(ctx : &egui::Context, scale: f32) {
    let mut fonts = egui::FontDefinitions::default(); 
    for (_name, data) in &mut fonts.font_data { 
        data.tweak.scale = scale;  
    }
    ctx.set_fonts(fonts);
}