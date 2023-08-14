mod app;
use app::Carbon;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(960.0, 504.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Carbon",
        options,
        Box::new(|_cc| Box::<Carbon>::default())
    )
}