use std::fs;

pub struct Cartridge {

}

impl Cartridge {
    pub fn new(file_name: &str) -> Self {
        let bytes = fs::read(file_name).expect("Cannot find file");

        Cartridge {}
    }

    /// Read from the Main Bus
    pub fn cpu_read(&mut self, address: u16, read_only: bool) -> u8 {
        0
    }

    /// Write to the Main Bus
    pub fn cpu_write(&mut self, address: u16, data: u8) {

    }

    /// Read from the PPU Bus
    pub fn ppu_read(&mut self, address: u16, read_only: bool) -> u8 {
        0
    }

    /// WRite to the PPU Bus
    pub fn ppu_write(&mut self, address: u16, data: u8) {

    }
}