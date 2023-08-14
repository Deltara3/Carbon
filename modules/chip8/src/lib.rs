use api::*;
mod chip8;
use chip8::*;

#[no_mangle]
pub fn load() -> Module {
    Module {
        core: Box::new(Chip8::new()),
        metadata: Metadata {
            name: "JACC",
            consoles: vec!["CHIP-8"],
            authors: vec!["Deltara"],
            version: [1, 0, 0],
            extensions: vec!["ch8"]
        },
        features: None
    }
}
