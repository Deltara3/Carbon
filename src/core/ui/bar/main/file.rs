use eframe::egui;
use crate::core::ui::{State, WindowState};
use rfd::FileDialog;
use crate::core::rom::{RomType, detect};
use crate::core::console::chip8::Chip8;
use crate::core::System;

pub fn render(state: &mut State, window_state: &mut WindowState, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    ui.menu_button("🗄 File", |ui| {
        if ui.button("📂 Open").clicked() {
            let load = FileDialog::new()
                .add_filter("CHIP-8", &["ch8"])
                .pick_file();
            
            if let Some(path) = load {
                state.unload();
                let (romname, romtype) = detect(path.clone());

                match romtype {
                    RomType::Chip => {
                        if let Ok(chip) = Chip8::new(path.clone()) { state.chip.console = Some(chip); }

                        if state.chip.console.is_some() { state.emulating = true; }
                        frame.set_window_title(&format!("Carbon - {} (CHIP-8)", romname));
                        state.sys = System::Chip;
                    },
                    RomType::Invalid => { /* Stop loading */ }
                }

                state.cur_rom = Some(path);
            };

            ui.close_menu();
        }

        let enable_buttons = if state.cur_rom.is_some() { true } else { false };
        let alt_text = match state.emulating { true => "⏸ Pause", false => "▶ Continue" };

        if ui.add_enabled(enable_buttons, egui::Button::new(alt_text)).clicked() { 
            state.emulating = !state.emulating;
        }

        if ui.add_enabled(enable_buttons, egui::Button::new("⟲ Reset")).clicked() {
            if let Some(path) = &state.cur_rom {
                if let Ok(chip) = Chip8::new(path.to_path_buf()) {
                    state.chip.console = Some(chip);
                }
            }
        }

        if ui.add_enabled(enable_buttons, egui::Button::new("🗙 Close")).clicked() {
            state.unload();
        }

        ui.separator();

        if ui.button("⎆ Quit").clicked() { frame.close(); }
    });
}