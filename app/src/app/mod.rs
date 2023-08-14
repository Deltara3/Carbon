mod bar;

pub struct Carbon;

impl Default for Carbon {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for Carbon {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        /* Add icon font to fonts */
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Bold);
        ctx.set_fonts(fonts);

        /* TODO: Make theme switchable if you want */
        let mut visual = egui::Visuals::dark();
        visual.window_shadow = egui::epaint::Shadow::NONE;
        visual.popup_shadow = egui::epaint::Shadow::NONE;
        ctx.set_visuals(visual);

        /* Rendering and logic */
        bar::main(ctx, frame);

        ctx.request_repaint();
    }
}