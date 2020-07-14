pub trait Mapper {
    fn reset(&mut self);
    fn get_prg_banks(&mut self) -> u8;
    fn get_chr_banks(&mut self) -> u8;
    fn cpu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool;
    fn cpu_map_write(&mut self, address: u16, mapped_address: &mut u32) -> bool;
    fn ppu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool;
    fn ppu_map_write(&mut self, address: u16, mapped_address: &mut u32) -> bool;
}