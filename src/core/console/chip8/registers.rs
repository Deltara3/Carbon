pub struct Registers {
    pub v: [u8; 16],
    pub stack: [u16; 16],
    pub i: usize,
    pub pc: usize,
    pub sp: u8,
    pub st: u8,
    pub dt: u8,
    pub key: usize
}

impl Registers {
    pub fn new() -> Registers {
        return Registers {
            v: [0x00; 16],
            stack: [0x0000; 16],
            i: 0x0000,
            pc: 0x0200,
            sp: 0x00,
            st: 0x00,
            dt: 0x00,
            key: 0x00
        }
    }
}