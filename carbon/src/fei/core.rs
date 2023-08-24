use super::{Handler, Driver};

/* Will implement core functionality like Netplay */
pub trait Core: Handler + Driver {
    fn load(&mut self, rom: Vec<u8>);
}