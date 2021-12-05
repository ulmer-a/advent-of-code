#[derive(Copy, Clone)]
pub struct BitPushNumber {
    number: u32,
}

impl BitPushNumber {
    pub fn new() -> BitPushNumber {
        BitPushNumber { number: 0 }
    }

    pub fn push(&mut self, bit: u32) {
        self.number = (self.number | bit) << 1;
    }

    pub fn get(&self) -> u32 {
        self.number >> 1
    }
}
