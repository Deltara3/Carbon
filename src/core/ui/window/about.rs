use eframe::egui;
use eframe::egui::os::OperatingSystem;
use egui::special_emojis::{OS_WINDOWS, OS_APPLE, OS_LINUX};
use crate::core::ui::State;
use crate::VERSION;

pub fn render(state: &mut State, ctx: &egui::Context) {
    egui::Window::new("ℹ About")
        .open(&mut state.about)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.label("Carbon is a work in progress multi-system emulator.");
            ui.label("Made with ♡ in Rust.");
            ui.separator();
            let info = format!("Version {} on {}", VERSION, match ctx.os() {
                OperatingSystem::Windows => OS_WINDOWS,
                OperatingSystem::Mac => OS_APPLE,
                OperatingSystem::Nix => OS_LINUX,
                _ => '❓'
            });
            ui.label(egui::RichText::new(info).weak().size(10.0));
        });
}