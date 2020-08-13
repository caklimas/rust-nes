use std::fmt::{Debug, Formatter, Result};
use crate::cartridge::mirror::Mirror;
use super::mapper_save_data::{MapperSaveData};
use super::mapper_results::{MapperReadResult, MapperWriteResult};

pub trait Mapper {
    fn reset(&mut self);
    fn get_prg_banks(&self) -> u8;
    fn get_chr_banks(&self) -> u8;
    fn get_mirror(&self) -> Mirror;
    fn irq_active(&self) -> bool;
    fn irq_clear(&mut self);
    fn irq_scanline(&mut self);
    fn cpu_map_read(&self, address: u16) -> MapperReadResult;
    fn cpu_map_write(&mut self, address: u16, data: u8) -> MapperWriteResult;
    fn ppu_map_read(&self, address: u16) -> MapperReadResult;
    fn ppu_map_write(&mut self, address: u16, mapped_address: &mut u32, data: u8) -> bool;
    fn load_battery_backed_ram(&mut self, data: Vec<u8>);
    fn save_battery_backed_ram(&self, file_path: &str);
    fn save_state(&self) -> MapperSaveData;
}

impl Debug for dyn Mapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Mapper")
         .field("prg_banks", &self.get_prg_banks())
         .field("chr_banks", &self.get_chr_banks())
         .finish()
    }
}