use crate::core::rom::raw;
use std::io::Result;
use std::path::PathBuf;
use crate::core::disasm::chip8::{DataType, disasm};

pub struct Rom {
    pub data: Vec<u8>,
    pub disassembly: Vec<DataType>,
    pub sprite_data: Vec<DataType>
}

impl Rom {
    pub fn new(location: PathBuf) -> Result<Rom> {
        let data = raw(location)?;
        let handled = disasm(data.clone());

        return Ok(Rom {
            data: data,
            disassembly: handled.0,
            sprite_data: handled.1
        });
    }
}