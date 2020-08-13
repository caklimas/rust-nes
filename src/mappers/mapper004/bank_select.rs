use serde::{Serialize, Deserialize};

use crate::memory_sizes::*;

const BANK_REGISTER_LENGTH: usize = 8;
const CHR_BANK_LENGTH: usize = 8;
const PRG_BANK_LENGTH: usize = 4;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct BankSelect {
    pub chr_banks: [u32; CHR_BANK_LENGTH],
    pub prg_banks: [u32; PRG_BANK_LENGTH],
    bank_registers: [u32; BANK_REGISTER_LENGTH],
    bank_register_index: usize,
    chr_inversion: ChrA12Inversion,
    number_prg_banks: u8,
    prg_bank_mode: PrgBankMode
}

impl BankSelect {
    pub fn new(number_prg_banks: u8) -> Self {
        BankSelect {
            chr_banks: [0; CHR_BANK_LENGTH],
            prg_banks: [0; PRG_BANK_LENGTH],
            bank_registers: [0; BANK_REGISTER_LENGTH],
            bank_register_index: 0,
            chr_inversion: ChrA12Inversion::LowerTwo2KilobyteBanks,
            number_prg_banks,
            prg_bank_mode: PrgBankMode::LowerSwappable
        }
    }

    pub fn select_bank(&mut self, data: u8) {
        self.bank_register_index = (data & 0b111) as usize;
        self.set_prg_bank_mode(data);
        self.set_chr_inversion(data);
    }

    pub fn set_bank_data(&mut self, data: u8) {
        self.bank_registers[self.bank_register_index] = data as u32;
        self.set_prg_banks();
        self.set_chr_banks();
    }

    pub fn reset(&mut self) {
        self.bank_register_index = 0;
        self.bank_registers = [0; BANK_REGISTER_LENGTH];
        self.chr_banks = [0; CHR_BANK_LENGTH];
        self.chr_inversion = ChrA12Inversion::LowerTwo2KilobyteBanks;
        self.prg_bank_mode = PrgBankMode::LowerSwappable;
        self.prg_banks = [0; PRG_BANK_LENGTH];

        self.prg_banks[0] = 0;
        self.prg_banks[1] = KILOBYTES_8 as u32;
        self.prg_banks[2] = ((self.number_prg_banks as u32) * 2 - 2) * KILOBYTES_8 as u32;
        self.prg_banks[3] = ((self.number_prg_banks as u32) * 2 - 1) * KILOBYTES_8 as u32;
    }

    fn set_chr_banks(&mut self) {
        let chr_offset = KILOBYTES_1 as u32;
        let ignore_bottom_bit = 0xFE;
        match self.chr_inversion {
            ChrA12Inversion::LowerTwo2KilobyteBanks => {
                self.chr_banks[0] = (self.bank_registers[0] & ignore_bottom_bit) * chr_offset;
                self.chr_banks[1] = (self.bank_registers[0] * chr_offset) + chr_offset;
                self.chr_banks[2] = (self.bank_registers[1] & ignore_bottom_bit) * chr_offset;
                self.chr_banks[3] = (self.bank_registers[1] * chr_offset) + chr_offset;
                self.chr_banks[4] = self.bank_registers[2] * chr_offset;
                self.chr_banks[5] = self.bank_registers[3] * chr_offset;
                self.chr_banks[6] = self.bank_registers[4] * chr_offset;
                self.chr_banks[7] = self.bank_registers[5] * chr_offset;
            },
            ChrA12Inversion::UpperTwo2KilobyteBanks => {
                self.chr_banks[0] = self.bank_registers[2] * chr_offset;
                self.chr_banks[1] = self.bank_registers[3] * chr_offset;
                self.chr_banks[2] = self.bank_registers[4] * chr_offset;
                self.chr_banks[3] = self.bank_registers[5] * chr_offset;
                self.chr_banks[4] = (self.bank_registers[0] & ignore_bottom_bit) * chr_offset;
                self.chr_banks[5] = (self.bank_registers[0] * chr_offset) + chr_offset;
                self.chr_banks[6] = (self.bank_registers[1] & ignore_bottom_bit) * chr_offset;
                self.chr_banks[7] = (self.bank_registers[1] * chr_offset) + chr_offset;
            }
        }
    }

    fn set_chr_inversion(&mut self, data: u8) {
        self.chr_inversion = if data & 0b1000_0000 == 0 {
            ChrA12Inversion::LowerTwo2KilobyteBanks
        } else {
            ChrA12Inversion::UpperTwo2KilobyteBanks
        };
    }

    fn set_prg_banks(&mut self) {
        let prg_offset = KILOBYTES_8 as u32;
        let ignore_top_two_bits = 0x3F;
        let last_bank = ((self.number_prg_banks as u32) * 2) - 1;
        let second_last_bank = last_bank - 1;
        match self.prg_bank_mode {
            PrgBankMode::LowerSwappable => {
                self.prg_banks[0] = (self.bank_registers[6] & ignore_top_two_bits) * prg_offset;
                self.prg_banks[2] = second_last_bank * prg_offset;
            },
            PrgBankMode::UpperSwappable => {
                self.prg_banks[0] = second_last_bank * prg_offset;
                self.prg_banks[2] = (self.bank_registers[6] & ignore_top_two_bits) * prg_offset;
            }
        }

        self.prg_banks[1] = (self.bank_registers[7] & ignore_top_two_bits) * prg_offset;
        self.prg_banks[3] = last_bank * prg_offset;
    }

    fn set_prg_bank_mode(&mut self, data: u8) {
        self.prg_bank_mode = if data & 0b0100_0000 == 0 {
            PrgBankMode::LowerSwappable
        } else {
            PrgBankMode::UpperSwappable
        };
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum PrgBankMode {
    /// $8000-$9FFF swappable
    /// $C000-$DFFF fixed to second-last bank
    LowerSwappable,

    /// $C000-$DFFF swappable
    /// $8000-$9FFF fixed to second-last bank
    UpperSwappable
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum ChrA12Inversion {
    /// two 2 KB banks at $0000-$0FFF
    /// four 1 KB banks at $1000-$1FFF
    LowerTwo2KilobyteBanks,

    /// two 2 KB banks at $1000-$1FFF
    /// four 1 KB banks at $0000-$0FFF
    UpperTwo2KilobyteBanks
}