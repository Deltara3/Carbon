use eframe::egui;
use crate::core::ui::{State, WindowState};
mod file;
mod tools;

pub fn main(state: &mut State, window_state: &mut WindowState, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("Menu").exact_height(20.0).show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            file::render(state, window_state, ui, frame);
            tools::render(state, window_state, ui, frame);
            if ui.button("ℹ About").clicked() { window_state.about = true; }
        });
    });
}