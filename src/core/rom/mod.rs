use std::io::{Result, Read, BufReader};
use std::fs::File;
use std::path::PathBuf;
pub mod chip8;

pub enum RomType {
    Chip,
    Invalid
}

pub fn raw(location: PathBuf) -> Result<Vec<u8>> {
    let raw = File::open(location)?;
    let mut reader = BufReader::new(raw);
    let mut buffer = Vec::new();
    
    reader.read_to_end(&mut buffer)?;

    return Ok(buffer);
}

pub fn detect(location: PathBuf) -> (String, RomType) {
    let mut romtype = RomType::Invalid;
    let mut romname = String::new();

    if let Some(extension) = location.extension() {
        if let Some(converted) = extension.to_str() {
            romtype = match converted {
                "ch8" => RomType::Chip,
                _ => RomType::Invalid
            }
        };
    };

    if let Some(name) = location.file_stem() {
        if let Some(finalized) = name.to_str() {
            romname = String::from(finalized);
        };
    };

    return (romname, romtype);
}