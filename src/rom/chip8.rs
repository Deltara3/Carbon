pub struct Data {
    pub raw: Vec<u8>
}

impl Data {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            raw: data
        }
    }
}