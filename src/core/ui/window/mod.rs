mod about;
mod opcode;
mod disassembler;
use crate::core::ui::{State, WindowState};
use eframe::egui;

pub fn render(state: &mut State, window_state: &mut WindowState, ctx: &egui::Context) {
    about::render(window_state, ctx);
    opcode::render(state, window_state, ctx);
    disassembler::render(state, window_state, ctx);
}