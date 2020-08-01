use crate::cartridge::mirror::Mirror;
use super::mapper_results::{MapperReadResult, MapperWriteResult};

pub trait Mapper {
    fn reset(&mut self);
    fn get_prg_banks(&self) -> u8;
    fn get_chr_banks(&self) -> u8;
    fn get_mirror(&self) -> Mirror;
    fn cpu_map_read(&self, address: u16) -> MapperReadResult;
    fn cpu_map_write(&mut self, address: u16, data: u8) -> MapperWriteResult;
    fn ppu_map_read(&self, address: u16) -> MapperReadResult;
    fn ppu_map_write(&mut self, address: u16, mapped_address: &mut u32, data: u8) -> bool;
}