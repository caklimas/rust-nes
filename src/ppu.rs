use std::rc::Rc;
use std::cell::RefCell;
use ggez::graphics;
use ggez::graphics::Color;

use crate::cartridge;
use crate::memory_sizes;
use crate::addresses;

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

const TILE_WIDTH: u16 = 16;
const TILE_HEIGHT: u16 = 16;
const FRAME_WIDTH: u16 = 256;
const FRAME_HEIGHT: u16 = 240;



pub struct Olc2C02 {
    pub name_table: [[u8; memory_sizes::KILOBYTES_1 as usize]; 2], // A full name table is 1KB and the NES can hold 2 name tables
    pub pallete_table: [u8; 32],
    pub pattern_table: [[u8; memory_sizes::KILOBYTES_4 as usize]; 2],
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub nmi: bool,
    pub frame_complete: bool,
    pub colors: [Color; 0x40],
    scanline: i16,
    cycle: u16,
    status: u8,
    control: u8,
    mask: u8,
    address_latch: bool,
    ppu_data_buffer: u8,
    current_vram_address: u16,
    temp_vram_address: u16,
    fine_x_scroll: u8
}

impl Olc2C02 {
    pub fn new() -> Self {
        Olc2C02 {
            name_table: [[0; 1024]; 2],
            pallete_table: [0; 32],
            pattern_table: [[0; memory_sizes::KILOBYTES_4 as usize]; 2],
            cartridge: None,
            nmi: false,
            scanline: 0,
            cycle: 0,
            frame_complete: false,
            colors: get_colors(),
            status: 0,
            control: 0,
            mask: 0,
            address_latch: false,
            ppu_data_buffer: 0,
            current_vram_address: 0,
            temp_vram_address: 0,
            fine_x_scroll: 0
        }
    }
        
    pub fn clock(&mut self) {
        if self.scanline == -1 && self.cycle == 1 {
            self.set_status(Status2C02::VerticalBlank, false);
        }

        if self.scanline == 241 && self.cycle == 1 {
            self.set_status(Status2C02::VerticalBlank, true);
            if self.get_control(Control2C02::GenerateNmi) == 1 {
                self.nmi = true;
            }
        }

        if self.scanline >= -1 || self.scanline < 240 {
            if self.get_mask(Mask2C02::RenderBackground) || self.get_mask(Mask2C02::RenderSprite) {
                if self.scanline == 256 {
                    // If rendering is enabled, the PPU increments the vertical position in v.
                    // The effective Y scroll coordinate is incremented, which is a complex operation that will correctly skip the attribute table memory regions,
                    // and wrap to the next nametable appropriately.
                    self.increment_y();
                } else if self.scanline == 257 {
                    // If rendering is enabled, the PPU copies all bits related to horizontal position from t to v:
                    // v: ....F.. ...EDCBA = t: ....F.....EDCBA
                    let fedcba = self.temp_vram_address & 0x41F;
                    self.set_current_address(ScrollAddress::CoarseX0, ((fedcba >> 0) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::CoarseX1, ((fedcba >> 1) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::CoarseX2, ((fedcba >> 2) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::CoarseX3, ((fedcba >> 3) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::CoarseX4, ((fedcba >> 4) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::NameTableSelect0, ((fedcba >> 10) & 0x01) > 0);

                    self.current_vram_address |= fedcba;
                } else if self.scanline >= 280 && self.scanline <= 304 {
                    // If rendering is enabled, at the end of vblank, shortly after the horizontal bits are copied from t to v at dot 257, 
                    // the PPU will repeatedly copy the vertical bits from t to v from dots 280 to 304, completing the full initialization of v from t:
                    // v: IHGF.ED CBA..... = t: IHGF.ED CBA.....
                    let ihgfedcba = self.temp_vram_address & 0x7BE0;
                    self.set_current_address(ScrollAddress::CoarseY0, ((ihgfedcba >> 5) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::CoarseY1, ((ihgfedcba >> 6) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::CoarseY2, ((ihgfedcba >> 7) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::CoarseY3, ((ihgfedcba >> 8) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::CoarseY4, ((ihgfedcba >> 9) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::NameTableSelect1, ((ihgfedcba >> 11) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::FineYScroll0, ((ihgfedcba >> 12) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::FineYScroll1, ((ihgfedcba >> 13) & 0x01) > 0);
                    self.set_current_address(ScrollAddress::FineYScroll2, ((ihgfedcba >> 14) & 0x01) > 0);
                } else if (self.scanline >= 328 || (self.scanline != 0 && self.scanline <= 256)) && self.scanline % 8 == 0 {

                } 
            }
        }

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
        let mut data: u8 = 0;
        match address {
            CONTROL => (), // Can't be read
            MASK => (), // Can't be read
            STATUS => {
                data = (self.status & 0xE0) | (self.ppu_data_buffer & 0x1F);
                self.set_status(Status2C02::VerticalBlank, false);
                self.address_latch = false;
            },
            OAM_ADDRESS => (),
            OAM_DATA => (),
            SCROLL => (),
            PPU_ADDRESS => (),
            PPU_DATA => {
                data = self.ppu_data_buffer;
                self.ppu_data_buffer = self.ppu_read(self.current_vram_address, false);

                if self.current_vram_address > addresses::PALETTE_ADDRESS_LOWER {
                    data = self.ppu_data_buffer;
                }

                let address_increment = if self.get_control(Control2C02::VramAddress) == 1 { 32 } else { 1 };
                self.current_vram_address = self.current_vram_address.wrapping_add(address_increment);
            },
            _ => ()
        };

        data
    }

    /// Write to the Main Bus
    pub fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            CONTROL => {
                self.control = data;

                // t: ...BA.......... = d: ......BA
                let ba = (data & 0x03) as u16;
                self.set_temp_address(ScrollAddress::NameTableSelect0, (ba & 0x01) > 0);
                self.set_temp_address(ScrollAddress::NameTableSelect1, ((ba >> 1) & 0x01) > 0);
            },
            MASK => {
                self.mask = data;
            },
            STATUS => (),
            OAM_ADDRESS => (),
            OAM_DATA => (),
            SCROLL => {
                if !self.address_latch {
                    // t: ....... ...HGFED = d: HGFED...
                    // x:              CBA = d: .....CBA
                    // w:                  = 1
                    let hgfed = ((data & 0b11111000) >> 3) as u16;
                    self.set_temp_address(ScrollAddress::CoarseX0, ((hgfed >> 0) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseX1, ((hgfed >> 1) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseX2, ((hgfed >> 2) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseX3, ((hgfed >> 3) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseX4, ((hgfed >> 4) & 0x01) > 0);
                    self.fine_x_scroll = data & 0b111;
                    self.address_latch = true;
                } else {
                    // t: CBA..HGFED..... = d: HGFEDCBA
                    // w:                  = 0
                    let cba = (data & 0b111) as u16;
                    let hgfed = ((data & 0b11111000) as u16) >> 3;
                    self.set_temp_address(ScrollAddress::CoarseY0, ((hgfed >> 0) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseY1, ((hgfed >> 1) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseY2, ((hgfed >> 2) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseY3, ((hgfed >> 3) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseY4, ((hgfed >> 4) & 0x01) > 0);

                    self.set_temp_address(ScrollAddress::FineYScroll0, ((cba >> 0) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::FineYScroll1, ((cba >> 1) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::FineYScroll2, ((cba >> 2) & 0x01) > 0);

                    self.address_latch = false;
                }
            },
            PPU_ADDRESS => {
                if !self.address_latch {
                    // t: .FEDCBA........ = d: ..FEDCBA
                    // t: X.............. = 0
                    // w:                  = 1
                    let fedbca = data & 0b00111111;
                    self.set_temp_address(ScrollAddress::CoarseY3, ((fedbca >> 0) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseY4, ((fedbca >> 1) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::NameTableSelect0, ((fedbca >> 2) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::NameTableSelect1, ((fedbca >> 3) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::FineYScroll0, ((fedbca >> 4) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::FineYScroll1, ((fedbca >> 5) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::FineYScroll2, false);

                    self.address_latch = true;
                } else {
                    // t: ....... HGFEDCBA = d: HGFEDCBA
                    // v                   = t
                    // w:                  = 0
                    self.set_temp_address(ScrollAddress::CoarseX0, ((data >> 0) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseX1, ((data >> 1) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseX2, ((data >> 2) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseX3, ((data >> 3) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseX4, ((data >> 4) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseY0, ((data >> 5) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseY1, ((data >> 6) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::CoarseY2, ((data >> 7) & 0x01) > 0);

                    self.current_vram_address = self.temp_vram_address;
                    self.address_latch = false;
                }
            },
            PPU_DATA => {
                self.ppu_write(self.current_vram_address, data);

                let address_increment = if self.get_control(Control2C02::VramAddress) == 1 { 32 } else { 1 };
                self.current_vram_address = self.current_vram_address.wrapping_add(address_increment);
            },
            _ => ()
        };
    }

    /// Read from the PPU Bus
    pub fn ppu_read(&mut self, address: u16, read_only: bool) -> u8 {
        let mut data: u8 = 0;
        let ppu_address = address & addresses::PPU_ADDRESS_END;

        match self.cartridge {
            Some(ref mut c) => {
                if c.borrow_mut().ppu_read(ppu_address, &mut data) {
                    return data;
                }
            },
            None => ()
        };

        if ppu_address <= addresses::PATTERN_ADDRESS_UPPER {
            data = self.read_pattern_table_data(ppu_address);
        } else if ppu_address >= addresses::NAME_TABLE_ADDRESS_LOWER && ppu_address <= addresses::NAME_TABLE_ADDRESS_UPPER {
            data = self.read_name_table_data(ppu_address);
        } else if ppu_address >= addresses::PALETTE_ADDRESS_LOWER && ppu_address <= addresses::PALETTE_ADDRESS_UPPER {
            data = self.read_palette_table_data(ppu_address);
        }

        data
    }

    /// Write to the PPU Bus
    pub fn ppu_write(&mut self, address: u16, data: u8) {
        let ppu_address = address & addresses::PPU_ADDRESS_END;

        match self.cartridge {
            Some(ref mut c) => {
                if c.borrow_mut().ppu_write(address, data) {
                    return;
                }
            },
            None => ()
        };

        if ppu_address <= addresses::PATTERN_ADDRESS_UPPER {
            self.write_pattern_table_data(ppu_address, data);
        } else if ppu_address >= addresses::NAME_TABLE_ADDRESS_LOWER && ppu_address <= addresses::NAME_TABLE_ADDRESS_UPPER {
            self.write_name_table_data(ppu_address, data);
        } else if ppu_address >= addresses::PALETTE_ADDRESS_LOWER && ppu_address <= addresses::PALETTE_ADDRESS_UPPER {
            self.write_palette_table_data(ppu_address, data);
        }
    }

    fn increment_y(&mut self) {
        // See https://wiki.nesdev.com/w/index.php/PPU_scrolling#Wrapping_around
        if (self.current_vram_address & 0x7000) != 0x7000 {
            self.current_vram_address += 0x1000; // If fine Y < 7 then increment fine Y
        } else {
            self.current_vram_address &= !(0x7000); // Set fine Y to 0
            let mut y = (self.current_vram_address & 0x03E0) >> 5; // Set y to coarse y
            if y == 29 { // 29 is the last row of tiles in the name table
                y = 0; // Set coarse Y to 0
                self.current_vram_address ^= 0x8000; // Switch vertical nametable
            } else if y == 31 { // Coarse Y can be set out of bounds and will wrap to 0
                y = 0; // Coarse Y is 0 and the nametable is not switched
            } else {
                y += 1; // Increment Coarse Y
            }

            self.current_vram_address = 
                (self.current_vram_address & !0x03E0) | (y << 5); // Put coarse Y back into address
        }
    }

    fn read_pattern_table_data(&mut self, address: u16) -> u8 {
        let page =(address & 0x1000) >> 12;
        self.pattern_table[page as usize][(address & 0x0FFF) as usize]
    }

    fn write_pattern_table_data(&mut self, address: u16, data: u8) {
        let page = (address & 0x1000) >> 12;
        self.pattern_table[page as usize][(address & 0x0FFF) as usize] = data;
    }
    
    fn read_name_table_data(&mut self, address: u16) -> u8 {
        let mut data: u8 = 0;
        let address_offset = (address & 0x03FF) as usize; // Offset by size of name table(1023)

        match self.cartridge {
            Some(ref mut c) => {
                match c.borrow_mut().mirror {
                    cartridge::Mirror::Vertical => {
                        if address <= 0x03FF {
                            data = self.name_table[0][address_offset];
                        } else if address >= 0x0400 && address <= 0x07FF {
                            data = self.name_table[1][address_offset];
                        } else if address >= 0x800 && address <= 0x0BFF {
                            data = self.name_table[0][address_offset];
                        } else if address >= 0x0C00 && address <= 0x0FFF {
                            data = self.name_table[1][address_offset];
                        }
                    },
                    cartridge::Mirror::Horizontal => {
                        if address <= 0x03FF {
                            data = self.name_table[0][address_offset];
                        } else if address >= 0x0400 && address <= 0x07FF {
                            data = self.name_table[0][address_offset];
                        } else if address >= 0x800 && address <= 0x0BFF {
                            data = self.name_table[1][address_offset];
                        } else if address >= 0x0C00 && address <= 0x0FFF {
                            data = self.name_table[1][address_offset];
                        }
                    }, 
                    _ => ()
                };
            },
            None => ()
        };

        data
    }

    fn write_name_table_data(&mut self, address: u16, data: u8) {
        let masked_address = address & 0x0FFF;
        let address_offset = (masked_address & 0x03FF) as usize; // Offset by size of name table(1023)

        match self.cartridge {
            Some(ref mut c) => {
                match c.borrow_mut().mirror {
                    cartridge::Mirror::Vertical => {
                        if masked_address <= 0x03FF {
                            self.name_table[0][address_offset] = data;
                        } else if masked_address >= 0x0400 && masked_address <= 0x07FF {
                            self.name_table[1][address_offset] = data;
                        } else if masked_address >= 0x800 && masked_address <= 0x0BFF {
                            self.name_table[0][address_offset] = data;
                        } else if masked_address >= 0x0C00 && masked_address <= 0x0FFF {
                            self.name_table[1][address_offset] = data;
                        }
                    },
                    cartridge::Mirror::Horizontal => {
                        if masked_address <= 0x03FF {
                            self.name_table[0][address_offset] = data;
                        } else if masked_address >= 0x0400 && masked_address <= 0x07FF {
                            self.name_table[0][address_offset] = data;
                        } else if masked_address >= 0x800 && masked_address <= 0x0BFF {
                            self.name_table[1][address_offset] = data;
                        } else if masked_address >= 0x0C00 && masked_address <= 0x0FFF {
                            self.name_table[1][address_offset] = data;
                        }
                    }, 
                    _ => ()
                };
            },
            None => ()
        };
    }

    fn read_palette_table_data(&mut self, address: u16) -> u8 {
        let mut masked_address = address & 0x001F;
        if masked_address == 0x0010 {
            masked_address = 0x0000;
        } else if masked_address == 0x0014 {
            masked_address = 0x0004;
        } else if masked_address == 0x0018 {
            masked_address = 0x0008;
        } else if masked_address == 0x001C {
            masked_address = 0x000C;
        }

        self.pallete_table[masked_address as usize]
    }

    fn write_palette_table_data(&mut self, address: u16, data: u8) {
        let mut masked_address = address & 0x001F;
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

    pub fn get_pattern_table(&mut self, pattern_index: u16, palette_id: u16) -> [[Color; 128]; 128] {
        let mut pattern_table: [[Color; 128]; 128] = [[graphics::BLACK; 128]; 128];

        for tile_y in 0..TILE_HEIGHT {
            for tile_x in 0..TILE_WIDTH {
                // We have 16 tiles which have 16 bytes of information
                let byte_offset = (tile_y * FRAME_WIDTH) + (tile_x * TILE_WIDTH);
                
                // Loop through 8 rows of 8 pixels
                for row in 0..8 {
                    let address = pattern_index * memory_sizes::KILOBYTES_4 + byte_offset + row;
                    let mut tile_lsb = self.ppu_read(address, false);
                    let mut tile_msb = self.ppu_read(address + 8, false);

                    // Now that we have the two bytes necessary, we need to loop through each bit and
                    // add them together to get the pixel index
                    for column in 0..8 {
                        let pixel = (tile_lsb & 0x01) + (tile_msb & 0x01);
                        let color = self.get_color_from_palette(palette_id, pixel as u16);
                        tile_lsb = tile_lsb >> 1;
                        tile_msb = tile_msb >> 1;

                        let y = (tile_y * 8) + row;
                        let x = (tile_x * 8) + (7 - column);

                        if x < 128 && y < 128 {
                            pattern_table[y as usize][x as usize] = color;
                        }
                    }
                }
            }
        }

        pattern_table
    }

    fn get_color_from_palette(&mut self, palette_id: u16, pixel_id: u16) -> Color {
        let address = addresses::PALETTE_ADDRESS_LOWER + (palette_id * 4) + pixel_id;
        let color_index = self.ppu_read(address, false) & 0x3F; // Make sure we don't go out of bounds
        self.colors[color_index as usize]
    }

    fn get_control(&mut self, control: Control2C02) -> u8 {
        match control {
            Control2C02::NameTableAddress => {
                return self.control & 0x03;
            },
            _ => {
                if self.control & (control as u8) > 0 {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn get_mask(&mut self, mask: Mask2C02) -> bool {
        self.status & (mask as u8) > 0
    }

    fn set_status(&mut self, status: Status2C02, value: bool) {
        if value {
            self.status = self.status | (status as u8);
        } else {
            self.status = self.status & !(status as u8);
        }
    }

    fn get_status(&mut self, status: Status2C02) -> u8 {
        if self.status & (status as u8) > 0 {
            1
        } else {
            0
        }
    }

    fn set_current_address(&mut self, scroll: ScrollAddress, value: bool) {
        if value {
            self.current_vram_address |= scroll as u16;
        } else {
            self.current_vram_address &= !(scroll as u16);
        }
    }

    fn set_temp_address(&mut self, scroll: ScrollAddress, value: bool) {
        if value {
            self.temp_vram_address |= scroll as u16;
        } else {
            self.temp_vram_address &= !(scroll as u16);
        }
    }
}

#[derive(Debug)]
pub enum Control2C02 {
    NameTableAddress =       0b00000011, // Base nametable address (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    VramAddress =            0b00000100, // VRAM address increment per CPU read/write of PPUDATA (0: add 1, going across; 1: add 32, going down)
    SpriteTableAddress =     0b00001000, // Sprite pattern table address for 8x8 sprites (0: $0000; 1: $1000; ignored in 8x16 mode)
    BackgroundTableAddress = 0b00010000, // Background pattern table address (0: $0000; 1: $1000)
    SpriteSize =             0b00100000, // (0: 8x8 pixels; 1: 8x16 pixels)
    PpuMasterSlaveSelect =   0b01000000, // PPU master/slave select (0: read backdrop from EXT pins; 1: output color on EXT pins)
    GenerateNmi =            0b10000000  // Generate an NMI at the start of the vertical blanking interval (0: off; 1: on)
}

#[derive(Debug)]
pub enum Mask2C02 {
    Greyscale =          0b00000001, // Greyscale (0: normal color, 1: produce a greyscale display)
    RenderBackgroundLeft = 0b00000010, // Show background in leftmost 8 pixels of screen, 0: Hide
    RenderSpriteLeft =     0b00000100, // Show sprites in leftmost 8 pixels of screen, 0: Hide
    RenderBackground =     0b00001000, // Show background
    RenderSprite =         0b00010000, // Show sprites
    EmphasizeRed =       0b00100000, // Emphasize red
    EmphasizeGreen =     0b01000000, // Emphasize green
    EMphasizeBlue =      0b10000000  // Emphasize blue
}

#[derive(Debug)]
pub enum Status2C02 {
    Unused = 0b00011111,
    SpriteOverflow = (1 << 5),
    SpriteZeroHit = (1 << 6),
    VerticalBlank = (1 << 7),
}

#[derive(Debug)]
pub enum ScrollAddress {
    CoarseX0 = (1 << 0),
    CoarseX1 = (1 << 1),
    CoarseX2 = (1 << 2),
    CoarseX3 = (1 << 3),
    CoarseX4 = (1 << 4),
    CoarseY0 = (1 << 5),
    CoarseY1 = (1 << 6),
    CoarseY2 = (1 << 7),
    CoarseY3 = (1 << 8),
    CoarseY4 = (1 << 9),
    NameTableSelect0 = (1 << 10),
    NameTableSelect1 = (1 << 11),
    FineYScroll0 = (1 << 12),
    FineYScroll1 = (1 << 13),
    FineYScroll2 = (1 << 14)
}

fn get_colors() -> [Color; 0x40] {
    [
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
    ]
}