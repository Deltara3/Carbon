#[derive(Debug)]
pub enum Opcode {
    CLS,
    RET,
    JP(usize, bool),
    CALL(usize),
    SE(usize, usize, u8, bool),
    SNE(usize, usize, u8, bool),
    LD(LdType),
    ADD(AddType),
    OR(usize, usize),
    AND(usize, usize),
    XOR(usize, usize),
    SUB(usize, usize),
    SHR(usize, usize),
    SUBN(usize, usize),
    SHL(usize, usize),
    RND(usize, u8),
    DRW(usize, usize, usize),
    SKP(usize),
    SKNP(usize),
    INVALID
}

#[derive(Debug)]
pub enum LdType {
    Simple(usize, usize, u8, bool),
    Addr(usize),
    K(usize),
    DT(usize, bool),
    ST(usize),
    F(usize),
    B(usize),
    I(usize, bool)
}

#[derive(Debug)]
pub enum AddType {
    Simple(usize, usize, u8, bool),
    I(usize)
}

impl Opcode {
    pub fn from_raw(opcode: u16) -> Opcode {
        let byte = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8
        );

        let nnn = (opcode & 0x0FFF) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let n = byte.3 as usize;
        let x = byte.1 as usize;
        let y = byte.2 as usize;

        return match byte {
            (0x00, 0x00, 0x0E, 0x00) => Opcode::CLS,
            (0x00, 0x00, 0x0E, 0x0E) => Opcode::RET,
            (0x01, _, _, _) => Opcode::JP(nnn, false),
            (0x02, _, _, _) => Opcode::CALL(nnn),
            (0x03, _, _, _) => Opcode::SE(x, y, nn, false),
            (0x04, _, _, _) => Opcode::SNE(x, y, nn, false),
            (0x05, _, _, 0x00) => Opcode::SE(x, y, nn, true),
            (0x06, _, _, _) => Opcode::LD(LdType::Simple(x, y, nn, false)),
            (0x07, _, _, _) => Opcode::ADD(AddType::Simple(x, y, nn, false)),
            (0x08, _, _, 0x00) => Opcode::LD(LdType::Simple(x, y, nn, true)),
            (0x08, _, _, 0x01) => Opcode::OR(x, y),
            (0x08, _, _, 0x02) => Opcode::AND(x, y),
            (0x08, _, _, 0x03) => Opcode::XOR(x, y),
            (0x08, _, _, 0x04) => Opcode::ADD(AddType::Simple(x, y, nn, true)),
            (0x08, _, _, 0x05) => Opcode::SUB(x, y),
            (0x08, _, _, 0x06) => Opcode::SHR(x, y),
            (0x08, _, _, 0x07) => Opcode::SUBN(x, y),
            (0x08, _, _, 0x0E) => Opcode::SHL(x, y),
            (0x09, _, _, 0x00) => Opcode::SNE(x, y, nn, true),
            (0x0A, _, _, _) => Opcode::LD(LdType::Addr(nnn)),
            (0x0B, _, _, _) => Opcode::JP(nnn, true),
            (0x0C, _, _, _) => Opcode::RND(x, nn),
            (0x0D, _, _, _) => Opcode::DRW(x, y, n),
            (0x0E, _, 0x09, 0x0E) => Opcode::SKP(x),
            (0x0E, _, 0x0A, 0x01) => Opcode::SKNP(x),
            (0x0F, _, 0x00, 0x07) => Opcode::LD(LdType::DT(x, false)),
            (0x0F, _, 0x00, 0x0A) => Opcode::LD(LdType::K(x)),
            (0x0F, _, 0x01, 0x05) => Opcode::LD(LdType::DT(x, true)),
            (0x0F, _, 0x01, 0x08) => Opcode::LD(LdType::ST(x)),
            (0x0F, _, 0x01, 0x0E) => Opcode::ADD(AddType::I(x)),
            (0x0F, _, 0x02, 0x09) => Opcode::LD(LdType::F(x)),
            (0x0F, _, 0x03, 0x03) => Opcode::LD(LdType::B(x)),
            (0x0F, _, 0x05, 0x05) => Opcode::LD(LdType::I(x, false)),
            (0x0F, _, 0x06, 0x05) => Opcode::LD(LdType::I(x, true)),
            _ => Opcode::INVALID
        }
    }
}