use eframe::egui;
use crate::core::ui::{State, WindowState};
use crate::core::System;

pub fn render(state: &mut State, window_state: &mut WindowState, ctx: &egui::Context) {
    egui::Window::new("⊗ Uh-oh!")
        .open(&mut window_state.uhoh)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.label("Carbon ran into an unknown or invalid opcode and had to stop.");
            ui.separator();
            let text = match state.sys {
                System::Chip => {
                    let mut buffer = String::new();
                    if let Some(chip) = &state.chip.console {
                        buffer = format!("CHIP-8: {:04X}", chip.get_raw_opcode());
                    }
                    buffer
                },
                System::Unloaded => String::from("This should not happen")
            };
            ui.label(text);
        });
}