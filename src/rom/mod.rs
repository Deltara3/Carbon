use anyhow::Result;
use std::io::{Read, BufReader};
use std::fs::File;
use std::path::PathBuf;
use std::ffi::OsStr;

pub enum Rom {

}

pub fn raw(location: PathBuf) -> Result<Vec<u8>> {
    let raw = File::open(location)?;
    let mut reader = BufReader::new(raw);
    let mut buffer = vec![];

    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn from(location: PathBuf) -> Option<Rom> {
    let ext = location
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or_default();

    let name = location
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("Unknown");

    let raw = raw(location.clone()).ok()?;

    match ext {
        _ => None
    }
}