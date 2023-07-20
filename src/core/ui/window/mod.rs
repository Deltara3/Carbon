mod about;
mod opcode;
use crate::core::ui::State;
use eframe::egui;

pub fn render(state: &mut State, ctx: &egui::Context) {
    about::render(state, ctx);
    opcode::render(state, ctx);
}