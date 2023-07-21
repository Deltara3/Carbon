use carbon::core::ui::app::Carbon;
use carbon::core::disasm;
use carbon::core::rom;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(960.0, 504.0)),
        ..Default::default()
    };

    return eframe::run_native(
        "Carbon",
        options,
        Box::new(|_cc| Box::<Carbon>::default())
    );
}