use super::mappers;
use crate::addresses::mappers::*;
use crate::memory_sizes;
use crate::cartridge::mirror::Mirror;


pub struct Mapper000 {
    pub prg_banks: u8,
    pub chr_banks: u8
}

impl Mapper000 {
    pub fn new(prg_banks: u8, chr_banks: u8) -> Self {
        Mapper000 {
            prg_banks,
            chr_banks
        }
    }
}

impl mappers::Mapper for Mapper000 {
    fn reset(&mut self) {}

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
        if address < CPU_MIN_ADDRESS {
            return false;
        }
        
        let prg_banks = self.get_prg_banks();
        let masked_address = if prg_banks > 1 { memory_sizes::KILOBYTES_32_MASK } else { memory_sizes::KILOBYTES_16_MASK };
        *mapped_address = (address & masked_address) as u32;

        true
    }

    fn cpu_map_write(&mut self, address: u16, mapped_address: &mut u32, data: u8) -> bool {
        if address < CPU_MIN_ADDRESS {
            return false;
        }

        let prg_banks = self.get_prg_banks();
        let masked_address = if prg_banks > 1 { memory_sizes::KILOBYTES_32_MASK } else { memory_sizes::KILOBYTES_16_MASK };
        *mapped_address = (address & masked_address) as u32;

        true
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