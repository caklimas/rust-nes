use crate::cartridge::mirror::Mirror;

pub trait Mapper {
    fn reset(&mut self);
    fn get_prg_banks(&self) -> u8;
    fn get_chr_banks(&self) -> u8;
    fn get_mirror(&self) -> Mirror;
    fn cpu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool;
    fn cpu_map_write(&mut self, address: u16, mapped_address: &mut u32, data: u8) -> bool;
    fn ppu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool;
    fn ppu_map_write(&mut self, address: u16, mapped_address: &mut u32, data: u8) -> bool;
}