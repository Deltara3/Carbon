use carbon::fei::*;
use super::Chip;

impl Handler for Chip {
    fn cycle(&mut self) -> Status {
        Status::Failure("Not implemented")
    }
}