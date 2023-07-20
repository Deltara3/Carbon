use crate::core::rom::raw;
use std::io::Result;
use std::path::PathBuf;

pub struct Rom {
    pub data: Vec<u8>
}

impl Rom {
    pub fn new(location: PathBuf) -> Result<Rom> {
        return Ok(Rom {
            data: raw(location)?
        });
    }
}