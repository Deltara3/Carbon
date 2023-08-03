mod font;
mod quirk;
mod registers;
use font::DEFAULT_FONT;
use registers::Registers;
use quirk::Quirk;
use crate::console::Console;

pub struct Chip8 {
    pub reg: Registers,
    pub vram: [[u8; 64]; 32],
    pub ram: [u8; 0x1000],
    keypad: [bool; 16],
    pub quirk: Quirk,
    waiting: bool
}