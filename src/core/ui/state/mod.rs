mod chip;
pub use chip::ChipState;
use std::path::PathBuf;
use crate::core::System;

pub struct WindowState {
    pub about: bool,
    pub uhoh: bool,
    pub disassembler: bool
}

impl WindowState {
    pub fn new() -> WindowState {
        return WindowState {
            about: false,
            uhoh: false,
            disassembler: false
        }
    }
}

pub struct State {
    pub emulating: bool,
    pub chip: ChipState,
    pub cur_rom: Option<PathBuf>,
    pub sys: System
}

impl State {
    pub fn new() -> State {
        return State {
            emulating: false,
            chip: ChipState::new(),
            cur_rom: None,
            sys: System::Unloaded
        }
    }

    pub fn unload(&mut self) {
        self.sys = System::Unloaded;
        self.cur_rom = None;
        if let Some(chip) = &self.chip.console { drop(chip); }
        self.chip.console = None;
    }
}