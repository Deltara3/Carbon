use carbon::fei::*;
mod chip;
use chip::*;

#[no_mangle]
pub fn init() -> Module {
    Module {
        core: Box::new(Chip::new()),
        metadata: Metadata {
            name: "Crisp",
            version: env!("CARGO_PKG_VERSION"),
            authors: vec!["Deltara"],
            consoles: vec!["CHIP-8"]
        },
        extensions: vec!["ch8"],
        features: None
    }
}