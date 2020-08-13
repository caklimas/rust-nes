use serde::{Serialize, Deserialize};

use crate::memory_sizes::*;
use super::control_register;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct ChrBank {
    pub chunk_4_low: u8,
    pub chunk_4_high: u8,
    pub chunk_8: u8
}

impl ChrBank {
    pub fn new() -> Self {
        ChrBank {
            chunk_4_low: 0,
            chunk_4_high: 0,
            chunk_8: 0
        }
    }

    pub fn reset(&mut self) {
        self.chunk_4_low = 0;
        self.chunk_4_high = 0;
        self.chunk_8 = 0;
    }

    pub fn get_mapped_address(&self, address: u16, mode: &control_register::ChrBankMode) -> u32 {
        match mode {
            control_register::ChrBankMode::Switch8KB => {
                (self.chunk_8 as u32) * (KILOBYTES_8 as u32) + ((address & KILOBYTES_8_MASK) as u32)
            },
            control_register::ChrBankMode::SwitchTwo4KB => {
                let select_4 = match address {
                    super::CHR_ROM_FIRST_BANK_LOWER..=super::CHR_ROM_FIRST_BANK_UPPER => {
                        self.chunk_4_low
                    },
                    super::CHR_ROM_LAST_BANK_LOWER..=super::CHR_ROM_LAST_BANK_UPPER => {
                        self.chunk_4_high
                    },
                    _ => panic!("Invalid CHR ROM mapping")
                };

                (select_4 as u32) * (KILOBYTES_4 as u32) + ((address & KILOBYTES_4_MASK) as u32)
            }
        }
    }

    pub fn write_low(&mut self, mode: &control_register::ChrBankMode, data: u8) {
        match mode {
            control_register::ChrBankMode::Switch8KB => self.chunk_8 = data & 0b11110,
            control_register::ChrBankMode::SwitchTwo4KB => self.chunk_4_low = data & 0b11111
        }
    }

    pub fn write_high(&mut self, mode: &control_register::ChrBankMode, data: u8) {
        match mode {
            control_register::ChrBankMode::Switch8KB => (), // Ignored in 8KB mode
            control_register::ChrBankMode::SwitchTwo4KB => self.chunk_4_high = data & 0b11111
        }
    }
}