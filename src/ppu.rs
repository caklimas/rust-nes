use std::rc::Rc;
use std::cell::RefCell;
use ggez::graphics::Color;

use crate::cartridge;
use crate::memory;

const CONTROL: u16 = 0x0000; // Configure ppu to render in different ways
const MASK: u16 = 0x0001; // Decides what sprites or backgrounds are being drawn and what happens at the edges of the screen
const STATUS: u16 = 0x0002;
const OAM_ADDRESS: u16 = 0x0003;
const OAM_DATA: u16 = 0x0004;
const SCROLL: u16 = 0x0005; // Used for worlds larger than the current screen
const PPU_ADDRESS: u16 = 0x0006; // The ppu address to send data to
const PPU_DATA: u16 = 0x0007; // The data to send to the ppu address
const MAX_CLOCK_CYCLE: u16 = 341;
const MAX_SCANLINE: i16 = 261;
const PATTERN_ADDRESS_UPPER: u16 = 0x1FFF;
const NAME_TABLE_ADDRESS_LOWER: u16 = 0x2000;
const NAME_TABLE_ADDRESS_UPPER: u16 = 0x3EFF;
const PALETTE_ADDRESS_LOWER: u16 = 0x3F00;
const PALETTE_ADDRESS_UPPER: u16 = 0x3FFF;

pub const PPU_ADDRESS_START: u16 = 0x2000;
pub const PPU_ADDRESS_END: u16 = 0x3FFF;
pub const PPU_ADDRESS_RANGE: u16 = 0x0007;

pub struct Olc2C02 {
    pub name_table: [[u8; 1024]; 2], // A full name table is 1KB and the NES can hold 2 name tables
    pub pallete_table: [u8; 32],
    pub pattern_table: [[u8; 4096]; 2],
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    scanline: i16,
    cycle: u16,
    frame_complete: bool,
    pal_screen: [Color; 0x40],
    status: Status2C02
}

impl Olc2C02 {
    pub fn new() -> Self {
        Olc2C02 {
            name_table: [[0; 1024]; 2],
            pallete_table: [0; 32],
            pattern_table: [[0; 4096]; 2],
            cartridge: None,
            scanline: 0,
            cycle: 0,
            frame_complete: false,
            pal_screen: [
                Color::from_rgb(84, 84, 84),
                Color::from_rgb(0, 30, 116),
                Color::from_rgb(8, 16, 144),
                Color::from_rgb(48, 0, 136),
                Color::from_rgb(68, 0, 100),
                Color::from_rgb(92, 0, 48),
                Color::from_rgb(84, 4, 0),
                Color::from_rgb(60, 24, 0),
                Color::from_rgb(32, 42, 0),
                Color::from_rgb(8, 58, 0),
                Color::from_rgb(0, 64, 0),
                Color::from_rgb(0, 60, 0),
                Color::from_rgb(0, 50, 60),
                Color::from_rgb(0, 0, 0),
                Color::from_rgb(0, 0, 0),
                Color::from_rgb(0, 0, 0),

                Color::from_rgb(152, 150, 152),
                Color::from_rgb(8, 76, 196),
                Color::from_rgb(48, 50, 236),
                Color::from_rgb(92, 30, 228),
                Color::from_rgb(136, 20, 176),
                Color::from_rgb(160, 20, 100),
                Color::from_rgb(152, 34, 32),
                Color::from_rgb(120, 60, 0),
                Color::from_rgb(84, 90, 0),
                Color::from_rgb(40, 114, 0),
                Color::from_rgb(8, 124, 0),
                Color::from_rgb(0, 118, 40),
                Color::from_rgb(0, 102, 120),
                Color::from_rgb(0, 0, 0),
                Color::from_rgb(0, 0, 0),
                Color::from_rgb(0, 0, 0),

                Color::from_rgb(236, 238, 236),
                Color::from_rgb(76, 154, 236),
                Color::from_rgb(120, 124, 236),
                Color::from_rgb(176, 98, 236),
                Color::from_rgb(228, 84, 236),
                Color::from_rgb(236, 88, 180),
                Color::from_rgb(236, 106, 100),
                Color::from_rgb(212, 136, 32),
                Color::from_rgb(160, 170, 0),
                Color::from_rgb(116, 196, 0),
                Color::from_rgb(76, 208, 32),
                Color::from_rgb(56, 204, 108),
                Color::from_rgb(56, 180, 204),
                Color::from_rgb(60, 60, 60),
                Color::from_rgb(0, 0, 0),
                Color::from_rgb(0, 0, 0),

                Color::from_rgb(236, 238, 236),
                Color::from_rgb(168, 204, 236),
                Color::from_rgb(188, 188, 236),
                Color::from_rgb(212, 178, 236),
                Color::from_rgb(236, 174, 236),
                Color::from_rgb(236, 174, 212),
                Color::from_rgb(236, 180, 176),
                Color::from_rgb(228, 196, 144),
                Color::from_rgb(204, 210, 120),
                Color::from_rgb(180, 222, 120),
                Color::from_rgb(168, 226, 144),
                Color::from_rgb(152, 226, 180),
                Color::from_rgb(160, 214, 228),
                Color::from_rgb(160, 162, 160),
                Color::from_rgb(0, 0, 0),
                Color::from_rgb(0, 0, 0)
            ],
            status: Status2C02::Unused(0)
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

        } else if ppu_address <= PATTERN_ADDRESS_UPPER {
            let page =(ppu_address & 0x1000) >> 12;
            data = self.pattern_table[page as usize][(ppu_address & 0x0FFF) as usize];
        } else if ppu_address >= NAME_TABLE_ADDRESS_LOWER && ppu_address <= NAME_TABLE_ADDRESS_UPPER {

        } else if ppu_address >= PALETTE_ADDRESS_LOWER && ppu_address <= PALETTE_ADDRESS_UPPER {
            let mut masked_address = ppu_address & 0x001F;
            if masked_address == 0x0010 {
                masked_address = 0x0000;
            } else if masked_address == 0x0014 {
                masked_address = 0x0004;
            } else if masked_address == 0x0018 {
                masked_address = 0x0008;
            } else if masked_address == 0x001C {
                masked_address = 0x000C;
            }

            data = self.pallete_table[masked_address as usize];
        }

        data
    }

    /// WRite to the PPU Bus
    pub fn ppu_write(&mut self, address: u16, data: u8) {
        let ppu_address = address & PPU_ADDRESS_END;

        if self.cartridge().borrow_mut().ppu_write(address, data) {
            return;
        } else if ppu_address <= PATTERN_ADDRESS_UPPER {
            let page = (ppu_address & 0x1000) >> 12;
            self.pattern_table[page as usize][(ppu_address & 0x0FFF) as usize];
        } else if ppu_address >= NAME_TABLE_ADDRESS_LOWER && ppu_address <= NAME_TABLE_ADDRESS_UPPER {

        } else if ppu_address >= PALETTE_ADDRESS_LOWER && ppu_address <= PALETTE_ADDRESS_UPPER {
            let mut masked_address = ppu_address & 0x001F;
            if masked_address == 0x0010 {
                masked_address = 0x0000;
            } else if masked_address == 0x0014 {
                masked_address = 0x0004;
            } else if masked_address == 0x0018 {
                masked_address = 0x0008;
            } else if masked_address == 0x001C {
                masked_address = 0x000C;
            }

            self.pallete_table[masked_address as usize] = data;
        }
    }

    fn cartridge(&mut self) -> Rc<RefCell<cartridge::Cartridge>> {
        self.cartridge.take().unwrap()
    }
}

pub enum Status2C02 {
    Unused(u8),
    SpriteOverflow(u8),
    SpriteZeroHit(u8),
    VerticalBlank(u8)
}