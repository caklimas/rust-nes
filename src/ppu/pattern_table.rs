use crate::memory_sizes::{KILOBYTES_4, KILOBYTES_4_MASK};

pub struct PatternTable {
    data: [[u8; KILOBYTES_4 as usize]; 2]
}

impl PatternTable {
    pub fn new() -> Self {
        PatternTable {
            data: [[0; KILOBYTES_4 as usize]; 2]
        }
    }

    pub fn read_data(&self, address: u16) -> u8 {
        let page =(address & KILOBYTES_4) >> 12;
        self.data[page as usize][(address & KILOBYTES_4_MASK) as usize]
    }

    pub fn write_data(&mut self, address: u16, data: u8) {
        let page = (address & KILOBYTES_4) >> 12;
        self.data[page as usize][(address & KILOBYTES_4_MASK) as usize] = data;
    }
}