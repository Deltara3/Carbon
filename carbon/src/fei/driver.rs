/* Implements all the I/O stuff */
pub trait Driver {
    fn video(&self);
    fn sound(&self);
    fn input(&mut self);
}