mod chip;
pub use chip::ChipState;
use std::path::PathBuf;
use crate::core::System;

pub struct State {
    pub emulating: bool,
    pub chip: ChipState,
    pub about: bool,
    pub uhoh: bool,
    pub cur_rom: Option<PathBuf>,
    pub sys: System
}

impl State {
    pub fn new() -> State {
        return State {
            emulating: false,
            chip: ChipState::new(),
            about: false,
            uhoh: false,
            cur_rom: None,
            sys: System::Unloaded
        }
    }

    pub fn unload(&mut self) {
        self.sys = System::Unloaded;
        self.cur_rom = None;
        if let Some(chip) = &self.chip.console { drop(chip); }
        self.chip.console = None;
        self.uhoh = false;
    }
}