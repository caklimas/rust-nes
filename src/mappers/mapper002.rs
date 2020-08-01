use super::mapper::{Mapper};
use super::mapper_results::{MapperReadResult, MapperWriteResult};
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

impl Mapper for Mapper002 {
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

    fn cpu_map_read(&self, address: u16) -> MapperReadResult {
        match address {
            CPU_MIN_ADDRESS..=SWITCHABLE_ROM_BANK_MAX => {
                let mapped_address = (self.prg_bank_low as u32) * (KILOBYTES_16 as u32) + ((address & KILOBYTES_16_MASK) as u32);
                return MapperReadResult::from_cart_ram(mapped_address);
            },
            FIXED_BANK_MIN..=CPU_MAX_ADDRESS => {
                let mapped_address = (self.prg_bank_high as u32) * (KILOBYTES_16 as u32) + ((address & KILOBYTES_16_MASK) as u32);
                return MapperReadResult::from_cart_ram(mapped_address);
            }
            _ => return MapperReadResult::none()
        }
    }

    fn cpu_map_write(&mut self, address: u16, data: u8) -> MapperWriteResult {
        if address >= CPU_MIN_ADDRESS {
            self.prg_bank_low = data & 0x0F;
        }

        MapperWriteResult::none()
    }

    fn ppu_map_read(&self, address: u16) -> MapperReadResult {
        match address {
            PPU_MIN_ADDRESS..=PPU_MAX_ADDRESS => MapperReadResult::from_cart_ram(address as u32),
            _ => MapperReadResult::none()
        }
    }

    fn ppu_map_write(&mut self, address: u16, mapped_address: &mut u32, data: u8) -> bool {
        
        if address > PPU_MAX_ADDRESS || self.get_chr_banks() != 0 {
            return false;
        }
        
        *mapped_address = address as u32;
        true
    }
}