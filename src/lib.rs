#[cfg(debug_assertions)]
pub const VERSION: char = eframe::egui::special_emojis::GITHUB;

#[cfg(not(debug_assertions))]
pub const VERSION: String = String::from(env!("CARGO_PKG_VERSION"));

pub mod core;