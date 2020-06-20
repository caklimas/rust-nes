const CONTROL: u16 = 0x0000;
const MASK: u16 = 0x0001;
const STATUS: u16 = 0x0002;
const OAM_ADDRESS: u16 = 0x0003;
const OAM_DATA: u16 = 0x0004;
const SCROLL: u16 = 0x0005;
const PPU_ADDRESS: u16 = 0x0006;
const PPU_DATA: u16 = 0x0007;

pub const PPU_ADDRESS_START: u16 = 0x2000;
pub const PPU_ADDRESS_END: u16 = 0x3FFF;
pub const PPU_ADDRESS_RANGE: u16 = 0x0007;

pub struct Olc2C02 {
    
}

impl Olc2C02 {
    pub fn new() -> Self {
        Olc2C02 {}
    }

    /// Read from the Main Bus
    pub fn cpu_read(&mut self, address: u16, read_only: bool) -> u8 {
        match address {
            CONTROL => (),
            MASK => (),
            STATUS => (),
            OAM_ADDRESS => (),
            OAM_DATA => (),
            SCROLL => (),
            PPU_ADDRESS => (),
            PPU_DATA => (),
            _ => ()
        }

        0
    }

    /// Write to the Main Bus
    pub fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            CONTROL => (),
            MASK => (),
            STATUS => (),
            OAM_ADDRESS => (),
            OAM_DATA => (),
            SCROLL => (),
            PPU_ADDRESS => (),
            PPU_DATA => (),
            _ => ()
        }
    }

    /// Read from the PPU Bus
    pub fn ppu_read(&mut self, address: u16, read_only: bool) -> u8 {
        let data: u8 = 0;
        let ppu_address = address & PPU_ADDRESS_END;

        data
    }

    /// WRite to the PPU Bus
    pub fn ppu_write(&mut self, address: u16, data: u8) {
        let ppu_address = address & PPU_ADDRESS_END;
    }
}

pub enum Address2C02 {
    Control = 0x0000,
    Mask = 0x0001,
    Status = 0x0002,
    OamAddress = 0x0003,
    OamData = 0x0004,
    Scroll = 0x0005,
    PpuAddress = 0x0006,
    PpuData = 0x0007
}