#[macro_use]
extern crate dlopen_derive;

use dlopen::wrapper::{Container, WrapperApi};
use carbon::*;

#[derive(WrapperApi)]
struct FeiExtension {
    init: fn() -> Module
}

fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();

    let mut module: Container<FeiExtension> = unsafe {
        Container::load(path)
    }
    .unwrap();

    let test = module.init();
    let mut core = test.core;

    loop {
        let status = box_method!(core, cycle);

        match status {
            Status::Failure(res) => {
                println!("Failed with response: {}", res);
                break;
            },
            Status::Cycle(_) | Status::Success => { }
        }
    }
}
