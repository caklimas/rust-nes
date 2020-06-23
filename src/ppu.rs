use std::rc::Rc;
use std::cell::RefCell;
use crate::cartridge;

const CONTROL: u16 = 0x0000;
const MASK: u16 = 0x0001;
const STATUS: u16 = 0x0002;
const OAM_ADDRESS: u16 = 0x0003;
const OAM_DATA: u16 = 0x0004;
const SCROLL: u16 = 0x0005;
const PPU_ADDRESS: u16 = 0x0006;
const PPU_DATA: u16 = 0x0007;
const MAX_CLOCK_CYCLE: u8 = 341;
const MAX_SCANLINE: i8 = 261;

pub const PPU_ADDRESS_START: u16 = 0x2000;
pub const PPU_ADDRESS_END: u16 = 0x3FFF;
pub const PPU_ADDRESS_RANGE: u16 = 0x0007;

pub struct Olc2C02 {
    pub name_table: [[u8; 1024]; 2], // A full name table is 1KB and the NES can hold 2 name tables
    pub pallete_table: [u8; 32],
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    scanline: i8,
    cycle: u8,
    frame_complete: bool
}

impl Olc2C02 {
    pub fn new() -> Self {
        Olc2C02 {
            name_table: [[0; 1024]; 2],
            pallete_table: [0; 32],
            cartridge: None,
            scanline: 0,
            cycle: 0,
            frame_complete: false
        }
    }

    pub fn clock(&mut self) {
        self.cycle += 1;

        if self.cycle >= MAX_CLOCK_CYCLE {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= MAX_SCANLINE {
                self.scanline = -1;
                self.frame_complete = true;
            }
        }
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
        let mut data: u8 = 0;
        let ppu_address = address & PPU_ADDRESS_END;

        if self.cartridge().borrow_mut().ppu_read(ppu_address, &mut data) {

        }

        data
    }

    /// WRite to the PPU Bus
    pub fn ppu_write(&mut self, address: u16, data: u8) {
        let ppu_address = address & PPU_ADDRESS_END;

        if self.cartridge().borrow_mut().ppu_write(address, data) {
            return;
        }
    }

    fn cartridge(&mut self) -> Rc<RefCell<cartridge::Cartridge>> {
        self.cartridge.take().unwrap()
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