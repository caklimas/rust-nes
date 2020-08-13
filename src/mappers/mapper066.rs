use serde::{Serialize, Deserialize};

use super::mapper::{Mapper};
use super::mapper_save_data::{MapperSaveData, Mapper066SaveData};
use super::mapper_results::{MapperReadResult, MapperWriteResult};
use crate::addresses::mappers::*;
use crate::memory_sizes::*;
use crate::cartridge::mirror::Mirror;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mapper066 {
    pub prg_banks: u8,
    pub chr_banks: u8,
    battery_backed_ram: bool,
    chr_bank: u8,
    prg_bank: u8
}

impl Mapper066 {
    pub fn new(prg_banks: u8, chr_banks: u8, battery_backed_ram: bool) -> Self {
        Mapper066 {
            prg_banks,
            chr_banks,
            battery_backed_ram,
            chr_bank: 0,
            prg_bank: 0
        }
    }

    pub fn from(data: &Mapper066SaveData) -> Self {
        Mapper066 {
            prg_banks: data.prg_banks,
            chr_banks: data.chr_banks,
            battery_backed_ram: data.battery_backed_ram,
            chr_bank: data.chr_bank,
            prg_bank: data.prg_bank
        }
    }
}

impl Mapper for Mapper066 {
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
        
        MapperReadResult::from_cart_ram((self.prg_bank as u32) * (KILOBYTES_32 as u32) + ((address & KILOBYTES_32_MASK) as u32))
    }

    fn cpu_map_write(&mut self, address: u16, data: u8) -> MapperWriteResult {
        if address >= CPU_MIN_ADDRESS {
            self.chr_bank = data & 0b11;
            self.prg_bank = (data & 0b110000) >> 4;
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
        MapperSaveData::Mapper066(Mapper066SaveData {
            prg_banks: self.prg_banks,
            chr_banks: self.chr_banks,
            battery_backed_ram: self.battery_backed_ram,
            chr_bank: self.chr_bank,
            prg_bank: self.prg_bank
        })
    }
}