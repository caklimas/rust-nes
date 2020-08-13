use serde::{Serialize, Deserialize};
use std::fmt::{Debug, Formatter, Result};
use crate::memory_sizes::{KILOBYTES_4, KILOBYTES_4_MASK, KILOBYTES_8};

big_array! { BigArray; }

#[derive(Serialize, Deserialize)]
pub struct PatternTable {
    #[serde(with = "BigArray")]
    data: [u8; KILOBYTES_8 as usize]
}

impl PatternTable {
    pub fn new() -> Self {
        PatternTable {
            data: [0; KILOBYTES_8 as usize]
        }
    }

    pub fn read_data(&self, address: u16) -> u8 {
        let index = get_index(address);
        self.data[index]
    }

    pub fn write_data(&mut self, address: u16, data: u8) {
        let index = get_index(address);
        self.data[index] = data;
    }
}

fn get_index(address: u16) -> usize {
    let page = (address & KILOBYTES_4) >> 12;
    (page * KILOBYTES_4) as usize + (address & KILOBYTES_4_MASK) as usize
}

impl Debug for PatternTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("PatternTable")
         .field("data_length", &self.data.len())
         .finish()
    }
}