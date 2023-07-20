use eframe::egui;
use crate::core::ui::State;
mod file;

pub fn main(state: &mut State, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("Menu").exact_height(20.0).show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            file::render(state, ui, frame);
        });
    });
}