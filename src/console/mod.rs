mod c8;

pub trait Console {
    fn cycle(&mut self);
    fn step(&mut self);
    fn rb(&self, address: usize) -> u8;
    fn wb(&mut self, address: usize, value: u8);
    fn rw(&self, address: usize) -> u16;
    fn ww(&mut self, address: usize, value: u16);
}