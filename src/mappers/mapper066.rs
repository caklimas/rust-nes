use super::mappers;
use crate::addresses::mappers::*;
use crate::memory_sizes::*;
use crate::cartridge::mirror::Mirror;

#[derive(Debug)]
pub struct Mapper066 {
    pub prg_banks: u8,
    pub chr_banks: u8,
    chr_bank: u8,
    prg_bank: u8
}

impl Mapper066 {
    pub fn new(prg_banks: u8, chr_banks: u8) -> Self {
        Mapper066 {
            prg_banks,
            chr_banks,
            chr_bank: 0,
            prg_bank: 0
        }
    }
}

impl mappers::Mapper for Mapper066 {
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
        
        *mapped_address = (self.prg_bank as u32) * (KILOBYTES_32 as u32) + ((address & KILOBYTES_32_MASK) as u32);

        true
    }

    fn cpu_map_write(&mut self, address: u16, mapped_address: &mut u32, data: u8) -> bool {
        if address >= CPU_MIN_ADDRESS {
            self.chr_bank = data & 0b11;
            self.prg_bank = (data & 0b110000) >> 4;
            *mapped_address = address as u32;
        }

        false
    }

    fn ppu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool {
        if address > PPU_MAX_ADDRESS {
            return false;
        }

        *mapped_address = (self.chr_bank as u32) * (KILOBYTES_8 as u32) + ((address & KILOBYTES_8_MASK) as u32);
        true
    }

    fn ppu_map_write(&mut self, _address: u16, _mapped_address: &mut u32, _data: u8) -> bool {
        false
    }
}