use super::Status;

/* Implements console execution stuff */
pub trait Handler {
    fn cycle(&mut self) -> Status;
}