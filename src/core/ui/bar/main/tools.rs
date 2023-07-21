use eframe::egui;
use crate::core::ui::{State, WindowState};

pub fn render(state: &mut State, window_state: &mut WindowState, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    ui.menu_button("🔧 Tools", |ui| {
        if ui.checkbox(&mut window_state.disassembler, "📦 Disassembler").clicked() {
            ui.close_menu();
        }
    });
}