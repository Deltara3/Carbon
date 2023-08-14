mod main;

pub fn main(ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("Menu").exact_height(20.0).show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            main::module::render(ui, frame);
        });
    });
}