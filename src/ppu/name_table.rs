use std::rc::Rc;
use std::cell::RefCell;
use crate::cartridge::Cartridge;
use crate::cartridge::mirror::Mirror;
use crate::memory_sizes::*;

/// A full name table is 1KB and the NES can hold 2 name tables
pub struct NameTable {
    data: [[u8; KILOBYTES_1 as usize]; 2]
}

impl NameTable {
    pub fn new() -> Self {
        NameTable {
            data: [[0; KILOBYTES_1 as usize]; 2]
        }
    }
    
    pub fn read_data(&mut self, address: u16, cartridge: &Option<Rc<RefCell<Cartridge>>>) -> u8 {
        let name_table_address = NameTableAddress::new(address);
        match cartridge {
            Some(ref c) => {
                match c.borrow().get_mirror() {
                    Mirror::Vertical => {
                        match name_table_address.masked_address {
                            0x0000..=0x03FF => self.data[0][name_table_address.address_offset],
                            0x0400..=0x07FF => self.data[1][name_table_address.address_offset],
                            0x0800..=0x0BFF => self.data[0][name_table_address.address_offset],
                            0x0C00..=0x0FFF => self.data[1][name_table_address.address_offset],
                            _ => 0
                        }
                    },
                    Mirror::Horizontal => {
                        match name_table_address.masked_address {
                            0x0000..=0x07FF => self.data[0][name_table_address.address_offset],
                            0x0800..=0x0FFF => self.data[1][name_table_address.address_offset],
                            _ => 0
                        }
                    },
                    Mirror::OneScreenLow => {
                        self.data[0][name_table_address.address_offset]
                    },
                    Mirror::OneScreenHigh => {
                        self.data[1][name_table_address.address_offset]
                    }
                    _ => 0
                }
            }
            None => 0
        }
    }

    pub fn write_data(&mut self, address: u16, cartridge: &Option<Rc<RefCell<Cartridge>>>, data: u8) {
        let name_table_address = NameTableAddress::new(address);
        match cartridge {
            Some(ref c) => {
                match c.borrow().get_mirror() {
                    Mirror::Vertical => {
                        match name_table_address.masked_address {
                            0x0000..=0x03FF => self.data[0][name_table_address.address_offset] = data,
                            0x0400..=0x07FF => self.data[1][name_table_address.address_offset] = data,
                            0x0800..=0x0BFF => self.data[0][name_table_address.address_offset] = data,
                            0x0C00..=0x0FFF => self.data[1][name_table_address.address_offset] = data,
                            _ => ()
                        }
                    },
                    Mirror::Horizontal => {
                        match name_table_address.masked_address {
                            0x0000..=0x07FF => self.data[0][name_table_address.address_offset] = data,
                            0x0800..=0x0FFF => self.data[1][name_table_address.address_offset] = data,
                            _ => ()
                        }
                    },
                    Mirror::OneScreenLow => {
                        self.data[0][name_table_address.address_offset] = data
                    },
                    Mirror::OneScreenHigh => {
                        self.data[1][name_table_address.address_offset] = data
                    }
                    _ => ()
                }
            },
            None => ()
        }
    }
}

struct NameTableAddress {
    address_offset: usize,
    masked_address: u16
}

impl NameTableAddress {
    fn new(address: u16) -> Self {
        let masked_address = address & 0x0FFF;
        let address_offset = (masked_address & KILOBYTES_1_MASK) as usize; // Offset by size of name table(1023)

        NameTableAddress {
            masked_address,
            address_offset
        }
    }
}