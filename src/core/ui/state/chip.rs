use std::time::{SystemTime, UNIX_EPOCH};
use crate::core::console::chip8::Chip8;
use eframe::egui::Color32;

pub struct ChipState {
    pub console: Option<Chip8>,
    pub last: Option<u128>,
    pub cps: u128,
    pub fixed_cps: f32,
    pub colors: [Color32; 2]
}

impl ChipState {
    pub fn new() -> ChipState {
        let mut last = None;
        if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
            last = Some(now.as_millis());
        }

        return ChipState {
            console: None,
            last: last,
            cps: 8,
            fixed_cps: 8.0,
            colors: [Color32::BLACK, Color32::WHITE] 
        }
    }

    pub fn update(&mut self) {
        if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
            let milli = now.as_millis();
            if let Some(last) = self.last {
                self.cps = (milli - last) * 500 / 1000;
                self.last = Some(milli);
            }
        }
    }
}