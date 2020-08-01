use crate::memory_sizes::*;
use super::control_register;

#[derive(Debug)]
pub struct PrgBank {
    pub chunk_16_low: u8,
    pub chunk_32: u8,
    pub chunk_16_high: u8,
    prg_banks: u8
}

impl PrgBank {
    pub fn new(prg_banks: u8) -> Self {
        PrgBank {
            prg_banks,
            chunk_16_low: 0,
            chunk_16_high: 0,
            chunk_32: 0
        }
    }

    pub fn reset(&mut self) {
        self.chunk_16_low = 0;
        self.chunk_16_high = self.prg_banks - 1;
        self.chunk_32 = 0;
    }

    pub fn get_mapped_address(&self, address: u16, prg_mode: &control_register::PrgBankMode) -> u32 {
        let mapped_address = match prg_mode {
            control_register::PrgBankMode::Switch32KB => (self.chunk_32 as u32) * (KILOBYTES_32 as u32) + ((address & KILOBYTES_32_MASK) as u32),
            control_register::PrgBankMode::FixFirst | control_register::PrgBankMode::FixLast => {
                let select_16 = match address {
                    super::PRG_ROM_FIRST_BANK_LOWER..=super::PRG_ROM_FIRST_BANK_UPPER => {
                        self.chunk_16_low
                    },
                    super::PRG_ROM_LAST_BANK_LOWER..=super::PRG_ROM_LAST_BANK_UPPER => {
                        self.chunk_16_high
                    },
                    _ => panic!("Invalid 16KB mapping")
                };

                (select_16 as u32) * (KILOBYTES_16 as u32) + ((address & KILOBYTES_16_MASK) as u32)
            }
        };

        mapped_address
    }

    pub fn write(&mut self, mode: &control_register::PrgBankMode, data: u8) {
        match mode {
            control_register::PrgBankMode::Switch32KB => self.chunk_32 = (data & 0b1110) >> 1,
            control_register::PrgBankMode::FixFirst => {
                self.chunk_16_low = 0;
                self.chunk_16_high = data & 0b1111
            },
            control_register::PrgBankMode::FixLast => {
                self.chunk_16_low = data & 0b1111;
                self.chunk_16_high = self.prg_banks - 1;
            }
        }
    }
}