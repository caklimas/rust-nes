use super::mappers;
use crate::addresses::mappers::*;
use crate::memory_sizes::{KILOBYTES_16, KILOBYTES_16_MASK};
use crate::cartridge::mirror::Mirror;

const SWITCHABLE_ROM_BANK_MAX: u16 = 0xBFFF;
const FIXED_BANK_MIN: u16 = 0xC000;

#[derive(Debug)]
pub struct Mapper002 {
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub prg_bank_low: u8,
    pub prg_bank_high: u8
}

impl Mapper002 {
    pub fn new(prg_banks: u8, chr_banks: u8) -> Self {
        Mapper002 {
            prg_banks,
            chr_banks,
            prg_bank_low: 0,
            prg_bank_high: if prg_banks > 0 { prg_banks - 1 } else { prg_banks }
        }
    }
}

impl mappers::Mapper for Mapper002 {
    fn reset(&mut self) {
        self.prg_bank_low = 0;
        self.prg_bank_high = self.prg_banks - 1;
    }

    fn get_prg_banks(&self) -> u8 {
        self.prg_banks
    }

    fn get_chr_banks(&self) -> u8 {
        self.chr_banks
    }

    fn get_mirror(&self) -> Mirror {
        Mirror::Hardware
    }

    fn cpu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool {
        if address >= CPU_MIN_ADDRESS && address <= SWITCHABLE_ROM_BANK_MAX {
            *mapped_address = ((self.prg_bank_low as u16) * KILOBYTES_16 + (address & KILOBYTES_16_MASK)) as u32;
            return true;
        } 
        
        if address >= FIXED_BANK_MIN && address <= CPU_MAX_ADDRESS {
            *mapped_address = ((self.prg_bank_high as u16) * KILOBYTES_16 + (address & KILOBYTES_16_MASK)) as u32;
            return true;
        };

        false
    }

    fn cpu_map_write(&mut self, address: u16, _mapped_address: &mut u32, data: u8) -> bool {
        if address >= CPU_MIN_ADDRESS {
            self.prg_bank_low = data & 0x0F;
        }

        false
    }

    fn ppu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool {
        if address > PPU_MAX_ADDRESS {
            return false;
        }

        *mapped_address = address as u32;
        true
    }

    fn ppu_map_write(&mut self, address: u16, mapped_address: &mut u32, data: u8) -> bool {
        
        if address > PPU_MAX_ADDRESS || self.get_chr_banks() != 0 {
            return false;
        }
        
        *mapped_address = address as u32;
        true
    }
}