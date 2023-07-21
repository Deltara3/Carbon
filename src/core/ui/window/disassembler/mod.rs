use eframe::egui;
use crate::core::ui::{State, WindowState};
use crate::core::System;
mod chip;

pub fn render(state: &mut State, window_state: &mut WindowState, ctx: &egui::Context) {
    egui::Window::new("📦 Disassembler")
        .open(&mut window_state.disassembler)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            match state.sys {
                System::Unloaded => { ui.label("Nothing currently loaded"); },
                System::Chip => chip::render(state, ui)
            }
        });
}