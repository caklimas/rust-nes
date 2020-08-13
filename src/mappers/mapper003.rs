use serde::{Serialize, Deserialize};

use super::mapper::{Mapper};
use super::mapper_save_data::{MapperSaveData, Mapper003SaveData};
use super::mapper_results::{MapperReadResult, MapperWriteResult};
use crate::addresses::mappers::*;
use crate::memory_sizes::*;
use crate::cartridge::mirror::Mirror;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mapper003 {
    pub prg_banks: u8,
    pub chr_banks: u8,
    battery_backed_ram: bool,
    chr_bank: u8
}

impl Mapper003 {
    pub fn new(prg_banks: u8, chr_banks: u8, battery_backed_ram: bool) -> Self {
        Mapper003 {
            prg_banks,
            chr_banks,
            battery_backed_ram,
            chr_bank: 0
        }
    }

    pub fn from(data: &Mapper003SaveData) -> Self {
        Mapper003 {
            prg_banks: data.prg_banks,
            chr_banks: data.chr_banks,
            battery_backed_ram: data.battery_backed_ram,
            chr_bank: data.chr_bank
        }
    }
}

impl Mapper for Mapper003 {
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

    fn irq_active(&self) -> bool { false }
    fn irq_clear(&mut self) {}
    fn irq_scanline(&mut self) {}

    fn cpu_map_read(&self, address: u16) -> MapperReadResult {
        if address < CPU_MIN_ADDRESS {
            return MapperReadResult::none();
        }
        
        let prg_banks = self.get_prg_banks();
        let masked_address = if prg_banks > 1 { KILOBYTES_32_MASK } else { KILOBYTES_16_MASK };
        MapperReadResult::from_cart_ram((address & masked_address) as u32)
    }

    fn cpu_map_write(&mut self, address: u16, data: u8) -> MapperWriteResult {
        if address >= CPU_MIN_ADDRESS {
            self.chr_bank = data & 0b11;
            return MapperWriteResult::with_mapped_address(address as u32);
        }

        MapperWriteResult::none()
    }

    fn ppu_map_read(&self, address: u16) -> MapperReadResult {
        match address {
            PPU_MIN_ADDRESS..=PPU_MAX_ADDRESS => {
                let mapped_address = (self.chr_bank as u32) * (KILOBYTES_8 as u32) + ((address & KILOBYTES_8_MASK) as u32);
                MapperReadResult::from_cart_ram(mapped_address)
            },
            _ => MapperReadResult::none()
        }
    }

    fn ppu_map_write(&mut self, _address: u16, _mapped_address: &mut u32, _data: u8) -> bool {
        false
    }

    fn load_battery_backed_ram(&mut self, _data: Vec<u8>) {}
    fn save_battery_backed_ram(&self, _file_path: &str) {}

    fn save_state(&self) -> MapperSaveData {
        MapperSaveData::Mapper003(Mapper003SaveData{
            prg_banks: self.prg_banks,
            chr_banks: self.chr_banks,
            battery_backed_ram: self.battery_backed_ram,
            chr_bank: self.chr_bank
        })
    }
}