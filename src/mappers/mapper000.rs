use super::mapper::{Mapper};
use super::mapper_results::{MapperReadResult, MapperWriteResult};
use crate::addresses::mappers::*;
use crate::memory_sizes;
use crate::cartridge::mirror::Mirror;

#[derive(Debug)]
pub struct Mapper000 {
    pub prg_banks: u8,
    pub chr_banks: u8,
    battery_backed_ram: bool
}

impl Mapper000 {
    pub fn new(prg_banks: u8, chr_banks: u8, battery_backed_ram: bool) -> Self {
        Mapper000 {
            prg_banks,
            chr_banks,
            battery_backed_ram
        }
    }
}

impl Mapper for Mapper000 {
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

    fn cpu_map_read(&self, address: u16) -> MapperReadResult {
        if address < CPU_MIN_ADDRESS {
            return MapperReadResult::none();
        }
        
        let prg_banks = self.get_prg_banks();
        let masked_address = if prg_banks > 1 { memory_sizes::KILOBYTES_32_MASK } else { memory_sizes::KILOBYTES_16_MASK };
        MapperReadResult::from_cart_ram((address & masked_address) as u32)
    }

    fn cpu_map_write(&mut self, address: u16, _data: u8) -> MapperWriteResult {
        if address < CPU_MIN_ADDRESS {
            return MapperWriteResult::none();
        }

        let prg_banks = self.get_prg_banks();
        let masked_address = if prg_banks > 1 { memory_sizes::KILOBYTES_32_MASK } else { memory_sizes::KILOBYTES_16_MASK };
        let mapped_address = (address & masked_address) as u32;

        MapperWriteResult::to_cart_ram(mapped_address)
    }

    fn ppu_map_read(&self, address: u16) -> MapperReadResult {
        match address {
            PPU_MIN_ADDRESS..=PPU_MAX_ADDRESS => MapperReadResult::from_cart_ram(address as u32),
            _ => MapperReadResult::none()
        }
    }

    fn ppu_map_write(&mut self, address: u16, mapped_address: &mut u32, _data: u8) -> bool {
        
        if address > PPU_MAX_ADDRESS || self.get_chr_banks() != 0 {
            return false;
        }
        
        *mapped_address = address as u32;
        true
    }

    fn load_battery_backed_ram(&mut self, _data: Vec<u8>) {}

    fn save_battery_backed_ram(&self, _file_path: &str) {}
}