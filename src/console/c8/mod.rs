mod font;
mod quirk;
mod registers;
mod opcode;
use font::DEFAULT_FONT;
use registers::Registers;
use quirk::Quirk;
use crate::console::Console;
use crate::rom::chip8::Data;
use opcode::*;
use rand::Rng;

pub struct Chip8 {
    pub reg: Registers,
    pub vram: [[u8; 64]; 32],
    pub ram: [u8; 0x1000],
    keypad: [bool; 16],
    pub quirk: Quirk,
    waiting: bool,
    pub op_error: bool
}

impl Console for Chip8 {
    fn cycle(&mut self) {

    }

    fn step(&mut self) {

    }

    fn rb(&self, address: usize) -> u8 {
        self.ram[address]
    }

    fn wb(&mut self, address: usize, value: u8) {
        self.ram[address] = value;
    }

    fn rw(&self, address: usize) -> u16 {
        (self.rb(address) as u16) << 8 | self.rb(address + 1) as u16
    }

    /* Unused in Chip8 */
    fn ww(&mut self, address: usize, value: u16) { }
}

impl Chip8 {
    pub fn new(rom: Data) -> Self {
        let mut ram = [0; 0x1000];

        for value in 0 .. DEFAULT_FONT.len() {
            ram[value] = DEFAULT_FONT[value];
        }

        for value in 0 .. rom.raw.len() {
            ram[value + 0x200] = rom.raw[value];
        }

        Self {
            reg: Registers::new(),
            vram: [[0; 64]; 32],
            ram: ram,
            keypad: [false; 16],
            quirk: Quirk::c8(),
            waiting: false,
            op_error: false
        }
    }
    
    pub fn decrement_timers(&mut self) {
        if self.reg.st > 0 { self.reg.st -= 1; }
        if self.reg.dt > 0 { self.reg.dt -= 1; }
    }

    pub fn execute(&mut self, raw: u16) {
        let opcode = Opcode::from_raw(raw);

        match opcode {
            Opcode::CLS => {
                for y in 0 .. 32 {
                    for x in 0 .. 64 {
                        self.vram[y][x] = 0;
                    }
                }
                self.reg.pc += 2;
            },
            Opcode::RET => {
                self.reg.sp -= 1;
                self.reg.pc = (self.reg.stack[self.reg.sp as usize]) as usize;
            },
            Opcode::JP(nnn, v0) => {
                let add = match v0 {
                    true => self.reg.v[0],
                    false => 0
                };

                self.reg.pc = nnn + add as usize;
            },
            Opcode::CALL(nnn) => {
                self.reg.stack[self.reg.sp as usize] = (self.reg.pc + 2) as u16;
                self.reg.sp += 1;
                self.reg.pc = nnn;
            },
            Opcode::SE(x, y, nn, swap) => {
                let value = match swap {
                    false => nn,
                    true => self.reg.v[y]
                };

                match self.reg.v[x] == value {
                    true => self.reg.pc += 4,
                    false => self.reg.pc += 2
                }
            }
            Opcode::SNE(x, y, nn, swap) => {
                let value = match swap {
                    false => nn,
                    true => self.reg.v[y]
                };

                match self.reg.v[x] != value {
                    true => self.reg.pc += 4,
                    false => self.reg.pc += 2
                }
            },
            Opcode::LD(ldtype) => {
                match ldtype {
                    LdType::Simple(x, y, nn, swap) => {
                        let value = match swap {
                            false => nn,
                            true => self.reg.v[y]
                        };

                        self.reg.v[x] = value;
                    },
                    LdType::Addr(nnn) => {
                        self.reg.i = nnn;
                    },
                    LdType::K(x) => {
                        self.waiting = true;
                        self.reg.key = x;
                    },
                    LdType::DT(x, swap) => {
                        match swap {
                            false => self.reg.v[x] = self.reg.dt,
                            true => self.reg.dt = self.reg.v[x]
                        }
                    },
                    LdType::ST(x) => {
                        self.reg.st = self.reg.v[x];
                    },
                    LdType::F(x) => {
                        self.reg.i = (self.reg.v[x] as usize) * 5;
                    },
                    LdType::B(x) => {
                        self.wb(self.reg.i, self.reg.v[x] / 100);
                        self.wb(self.reg.i + 1, (self.reg.v[x] % 100) / 10);
                        self.wb(self.reg.i + 2, self.reg.v[x] % 10);
                    },
                    LdType::I(x, swap) => {
                        for i in 0 .. x + 1 {
                            match swap {
                                false => self.wb(self.reg.i + i, self.reg.v[i]),
                                true => self.reg.v[i] = self.rb(self.reg.i + i)
                            }
                        }

                        if self.quirk.memory {
                            self.reg.i += 1;
                        }
                    }
                }

                self.reg.pc += 2;
            },
            Opcode::ADD(addtype) => {
                match addtype {
                    AddType::Simple(x, y, nn, swap) => {
                        match swap {
                            false => {
                                self.reg.v[x] = self.reg.v[x].wrapping_add(nn);
                            },
                            true => {
                                let (new, overflow) = self.reg.v[x].overflowing_add(self.reg.v[y]);
                                self.reg.v[x] = new;
                                self.reg.v[0x0F] = overflow as u8;
                            }
                        }
                    },
                    AddType::I(x) => {
                        self.reg.i += self.reg.v[x] as usize;
                        self.reg.v[0x0F] = if self.reg.i > 0x0F00 { 1 } else { 0 };
                    }
                }
                self.reg.pc += 2;
            },
            Opcode::OR(x, y) => {
                self.reg.v[x] |= self.reg.v[y];
                if self.quirk.vf_reset { self.reg.v[0x0F] = 0; }
                self.reg.pc += 2;
            },
            Opcode::AND(x, y) => {
                self.reg.v[x] &= self.reg.v[y];
                if self.quirk.vf_reset { self.reg.v[0x0F] = 0; }
                self.reg.pc += 2;
            },
            Opcode::XOR(x, y) => {
                self.reg.v[x] ^= self.reg.v[y];
                if self.quirk.vf_reset { self.reg.v[0x0F] = 0; }
                self.reg.pc += 2;
            },
            Opcode::SUB(x, y) => {
                let (new, overflow) = self.reg.v[x].overflowing_sub(self.reg.v[y]);
                self.reg.v[x] = new;
                self.reg.v[0x0F] = !overflow as u8;
                self.reg.pc += 2;
            },
            Opcode::SHR(x, y) => {
                let lsb = self.reg.v[x] & 1;
                if self.quirk.shifting { self.reg.v[x] >>= 1; } else { self.reg.v[x] = self.reg.v[y] >> 1; }
                self.reg.v[0x0F] = lsb;
                self.reg.pc += 2;
            },
            Opcode::SUBN(x, y) => {
                let (new, overflow) = self.reg.v[y].overflowing_sub(self.reg.v[x]);
                self.reg.v[x] = new;
                self.reg.v[0x0F] = !overflow as u8;
                self.reg.pc += 2;
            },
            Opcode::SHL(x, y) => {
                let msb = (self.reg.v[x] & 0b10000000) >> 7;
                if self.quirk.shifting { self.reg.v[x] <<= 1; } else { self.reg.v[x] = self.reg.v[y] << 1; }
                self.reg.v[0x0F] = msb;
                self.reg.pc += 2;
            },
            Opcode::RND(x, nn) => {
                let mut rng = rand::thread_rng();
                self.reg.v[x] = rng.gen::<u8>() & nn;
                self.reg.pc += 2;
            },
            Opcode::DRW(x, y, n) => {
                self.reg.v[0x0F] = 0;

                for byte in 0 .. n {
                    let y = if self.quirk.clipping {
                        (self.reg.v[y] as usize % 32) + byte
                    } else {
                        (self.reg.v[y] as usize + byte) % 32
                    };

                    if self.quirk.clipping && y >= 32 {
                        continue;
                    } else {
                        for bit in 0 .. 8 {
                            let x = if self.quirk.clipping {
                                (self.reg.v[x] as usize % 64) + bit
                            } else {
                                (self.reg.v[x] as usize + bit) % 64
                            };

                            if self.quirk.clipping && x >= 64 {
                                continue;
                            } else {
                                let color = (self.rb(self.reg.i + byte) >> (7 - bit)) & 1;
                                self.reg.v[0x0F] |= color & self.vram[y][x];
                                self.vram[y][x] ^= color;
                            }
                        }
                    }
                }

                self.reg.pc += 2;
            },
            Opcode::SKP(x) => {
                self.reg.pc += if self.keypad[self.reg.v[x] as usize] { 4 } else { 2 };
            },
            Opcode::SKNP(x) => {
                self.reg.pc += if !self.keypad[self.reg.v[x] as usize] { 4 } else { 2 };
            },
            Opcode::INVALID => { self.op_error = true; }
        }
    }
}