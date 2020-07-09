use std::rc::Rc;
use std::cell::RefCell;
use ggez::graphics::Color;

use crate::cartridge::cartridge;
use crate::memory_sizes;
use crate::addresses;
use crate::frame;
use crate::ppu::sprites;

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
const MAX_VISIBLE_SCANLINE: i16 = 239;
const MAX_VISIBLE_CLOCK_CYCLE: u16 = 257;

pub struct Ppu2C02 {
    pub name_table: [[u8; memory_sizes::KILOBYTES_1 as usize]; 2], // A full name table is 1KB and the NES can hold 2 name tables
    pub pallete_table: [u8; 32],
    pub pattern_table: [[u8; memory_sizes::KILOBYTES_4 as usize]; 2],
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub nmi: bool,
    pub frame_complete: bool,
    pub colors: Vec<Color>,
    pub frame: frame::Frame,
    pub oam: Vec<u8>,
    pub oam_address: u8,
    scanline: i16,
    cycle: u16,
    status: u8,
    control: u8,
    mask: u8,
    address_latch: bool,
    ppu_data_buffer: u8,
    current_vram_address: u16,
    temp_vram_address: u16,
    fine_x_scroll: u8,
    bg_next_tile_id: u8,
    bg_next_tile_attribute: u8,
    bg_next_tile_lsb: u8,
    bg_next_tile_msb: u8,
    bg_shifter_pattern_low: u16,
    bg_shifter_pattern_high: u16,
    bg_shifter_attribute_low: u16,
    bg_shifter_attribute_high: u16,
    sprite_scanline: Vec<u8>,
    sprite_count: usize,
    sprite_shifter_pattern_low: Vec<u8>,
    sprite_shifter_pattern_high: Vec<u8>,
    sprite_zero_hit_possible: bool,
    sprite_zero_being_rendered: bool
}

impl Ppu2C02 {
    pub fn new() -> Self {
        Ppu2C02 {
            name_table: [[0; 1024]; 2],
            pallete_table: [0; 32],
            pattern_table: [[0; memory_sizes::KILOBYTES_4 as usize]; 2],
            cartridge: None,
            nmi: false,
            scanline: 0,
            cycle: 0,
            frame_complete: false,
            colors: get_colors(),
            frame: frame::Frame::new(),
            oam: initialize_oam(),
            oam_address: 0x00,
            status: 0,
            control: 0,
            mask: 0,
            address_latch: false,
            ppu_data_buffer: 0,
            current_vram_address: 0,
            temp_vram_address: 0,
            fine_x_scroll: 0,
            bg_next_tile_id: 0x00,
            bg_next_tile_attribute: 0x00,
            bg_next_tile_lsb: 0x00,
            bg_next_tile_msb: 0x00,
            bg_shifter_pattern_low: 0x0000,
            bg_shifter_pattern_high: 0x0000,
            bg_shifter_attribute_low: 0x0000,
            bg_shifter_attribute_high: 0x0000,
            sprite_scanline: vec![0; sprites::MAX_SPRITE_COUNT * sprites::OAM_ENTRY_SIZE], // 8 sprites times size of an entry
            sprite_count: 0,
            sprite_shifter_pattern_low: vec![0; sprites::MAX_SPRITE_COUNT],
            sprite_shifter_pattern_high: vec![0; sprites::MAX_SPRITE_COUNT],
            sprite_zero_hit_possible: false,
            sprite_zero_being_rendered: false
        }
    }
   
    pub fn clock(&mut self) {
        if self.scanline >= -1 && self.scanline <= MAX_VISIBLE_SCANLINE {
            // Skipped on BG+odd
            if self.scanline == 0 && self.cycle == 0 {
                self.cycle = 1;
            }

            if self.scanline == -1 && self.cycle == 1 {
                self.set_status(Status2C02::VerticalBlank, false);
                self.set_status(Status2C02::SpriteOverflow, false);
                self.set_status(Status2C02::SpriteZeroHit, false);

                for i in 0..sprites::MAX_SPRITE_COUNT {
                    self.sprite_shifter_pattern_low[i] = 0;
                    self.sprite_shifter_pattern_high[i] = 0;
                }
            }

            self.render_background();
            self.render_foreground();
        }

        if self.scanline == 240 {
            // Post render scanline does nothing
        }

        if self.scanline >= 241 && self.scanline < 261 {
            if self.scanline == 241 && self.cycle == 1 {
                self.set_status(Status2C02::VerticalBlank, true);
                if self.get_control(Control2C02::GenerateNmi) == 1 {
                    self.nmi = true;
                }
            }
        }

        self.render_pixel();

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
    pub fn cpu_read(&mut self, address: u16, _read_only: bool) -> u8 {
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
            OAM_DATA => {
                data = self.oam[self.oam_address as usize];
            },
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
                let ba = (data & 0b11) as u16;
                self.set_temp_address(ScrollAddress::NameTableSelectX, (ba & 0x01) > 0);
                self.set_temp_address(ScrollAddress::NameTableSelectY, ((ba >> 1) & 0x01) > 0);
            },
            MASK => {
                self.mask = data;
            },
            STATUS => (),
            OAM_ADDRESS => {
                self.oam_address = data;
            },
            OAM_DATA => {
                self.oam[self.oam_address as usize] = data;
            },
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
                    self.set_temp_address(ScrollAddress::NameTableSelectX, ((fedbca >> 2) & 0x01) > 0);
                    self.set_temp_address(ScrollAddress::NameTableSelectY, ((fedbca >> 3) & 0x01) > 0);
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
    pub fn ppu_read(&mut self, address: u16, _read_only: bool) -> u8 {
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

    fn render_background(&mut self) {
        if (self.cycle >= 2 && self.cycle <= MAX_VISIBLE_CLOCK_CYCLE) || (self.cycle >= 321 && self.cycle < 338) {
            self.update_shifters();

            let sub_cycle = (self.cycle - 1) % 8;
            if sub_cycle == 0 {
                self.load_shifters();
                let name_table_address = addresses::NAME_TABLE_ADDRESS_LOWER | (self.current_vram_address & 0x0FFF);
                self.bg_next_tile_id = self.ppu_read(name_table_address, false);
            } else if sub_cycle == 2 {
                let coarse_x = self.get_coarse_x();
                let coarse_y = self.get_coarse_y();
                let name_table_x = self.get_current_address(ScrollAddress::NameTableSelectX) << 10;
                let name_table_y = self.get_current_address(ScrollAddress::NameTableSelectY) << 11;
                let attribute_table_address = 
                    addresses::ATTRIBUTE_TABLE_ADDRESS_LOWER |
                    name_table_y |
                    name_table_x |
                    (coarse_y >> 2) << 3 |
                    coarse_x >> 2;
                self.bg_next_tile_attribute = self.ppu_read(attribute_table_address, false);

                // Since there are only 4 palettes for the background tiles, we only need 2 bits to select a palette(2 bits range is 0-3)
                // We get a byte of data we can split that byte up into 4 sets of 2 bits.
                // One attribute byte covers a block of data (4x4 tiles) so we can apply each set of 2 bits to one quadrant of the block
                // Bits 7,6 => bottom right, Bits 5,4 => bottom left, Bits 3,2 => top right, Bits 1,0 => top left
                // If coarse y % 4 < 2 then it is in the top half
                // If coarse x % 4 < 2 then it is in the left half
                // Knowing this and that we want the last two bits to be the palette selected so we shift accordingly.
                if coarse_y & 0x02 > 0 {
                    self.bg_next_tile_attribute >>= 4; // Use bits 7,6 or 5,4
                }

                if coarse_x & 0x02 > 0 {
                    self.bg_next_tile_attribute >>= 2; // USe bits 7,6 or 3,2
                }

                self.bg_next_tile_attribute &= 0x03;
            } else if sub_cycle == 4 {
                let pattern_address = self.get_pattern_address(0);
                self.bg_next_tile_lsb = self.ppu_read(pattern_address, false);
            } else if sub_cycle == 6 {
                let pattern_address = self.get_pattern_address(8);
                self.bg_next_tile_msb = self.ppu_read(pattern_address, false);
            } else if sub_cycle == 7 {
                self.increment_x();
            }
        }

        if self.cycle == 256 {
            // If rendering is enabled, the PPU increments the vertical position in v.
            // The effective Y scroll coordinate is incremented, which is a complex operation that will correctly skip the attribute table memory regions,
            // and wrap to the next nametable appropriately.
            self.increment_y();
        }
        
        if self.cycle == MAX_VISIBLE_CLOCK_CYCLE {
            self.load_shifters();
            self.transfer_x_address();
        }

        if self.scanline == -1 && self.cycle >= 280 && self.cycle <= 304 {
            self.transfer_y_address();
        }

        // Useless read of the tile id at the end of the scanline
        if self.cycle == 338 || self.cycle == 340 {
            self.bg_next_tile_id = self.ppu_read(addresses::NAME_TABLE_ADDRESS_LOWER | (self.current_vram_address & 0x0FFF), false);
        }
    }

    fn render_foreground(&mut self) {
        // This isn't exactly how the NES does foreground rendering, however it gets there most of the way
        if self.cycle == MAX_VISIBLE_CLOCK_CYCLE && self.scanline >= 0 {
            self.evaluate_sprites();
        }

        if self.cycle == MAX_CLOCK_CYCLE - 1 {
            let sprite_mode = self.get_control(Control2C02::SpriteSize);
            for i in 0..self.sprite_count {
                let oam_entry = sprites::get_object_attribute_entry(&self.sprite_scanline, i * sprites::OAM_ENTRY_SIZE);
                let mut sprite_pattern_bit_low: u8;
                let mut sprite_pattern_bit_high: u8;
                let sprite_pattern_address_low: u16;
                let sprite_pattern_address_high: u16;
                let flip_vertically = oam_entry.get_oam_attribute(sprites::OamAttribute::FlipVertically) > 0;
                let flip_horizontally = oam_entry.get_oam_attribute(sprites::OamAttribute::FlipHorizontally) > 0;
                
                if sprite_mode == 0 {
                    if !flip_vertically {
                        sprite_pattern_address_low =
                            (self.get_control(Control2C02::SpriteTableAddress) as u16) << 12 |
                            ((oam_entry.tile_id as u16) << 4) |
                            ((self.scanline - (oam_entry.y as i16)) as u16);
                    } else {
                        sprite_pattern_address_low =
                            (self.get_control(Control2C02::SpriteTableAddress) as u16) << 12 |
                            ((oam_entry.tile_id as u16) << 4) |
                            (7 - ((self.scanline - (oam_entry.y as i16)) as u16));
                    }
                } else {
                    if !flip_vertically {
                        if self.scanline - (oam_entry.y as i16) < 8 {
                            sprite_pattern_address_low =
                                (((oam_entry.tile_id & 0x01) as u16) << 12) |
                                (((oam_entry.tile_id & 0xFE) as u16) << 4) |
                                (((self.scanline - (oam_entry.y as i16)) as u16 ) & 0x07);
                        } else {
                            sprite_pattern_address_low =
                                (((oam_entry.tile_id & 0x01) as u16) << 12) |
                                ((((oam_entry.tile_id & 0xFE) + 1) as u16) << 4) |
                                (((self.scanline - (oam_entry.y as i16)) as u16 ) & 0x07);
                        }
                    } else {
                        if self.scanline - (oam_entry.y as i16) < 8 {
                            sprite_pattern_address_low =
                                (((oam_entry.tile_id & 0x01) as u16) << 12) |
                                ((((oam_entry.tile_id & 0xFE) + 1) as u16) << 4) |
                                (((self.scanline - (oam_entry.y as i16)) as u16 ) & 0x07);
                        } else {
                            sprite_pattern_address_low =
                                (((oam_entry.tile_id & 0x01) as u16) << 12) |
                                (((oam_entry.tile_id & 0xFE) as u16) << 4) |
                                (((self.scanline - (oam_entry.y as i16)) as u16 ) & 0x07);
                        }
                    }
                }

                // let row = if flip_vertically { 
                //     7 - (self.scanline as u16) - (oam_entry.y as u16) 
                // } else { 
                //     (self.scanline as u16) - (oam_entry.y as u16) 
                // };

                // if sprite_mode == 0 {
                //     // We're in 8x8 pixel mode and the control register determines the pattern table
                //     sprite_pattern_address_low = 
                //         (self.get_control(Control2C02::SpriteTableAddress) as u16) << 12 |
                //         ((oam_entry.tile_id as u16) << 4) |
                //         row;
                // } else {
                //     // We're in 8x16 pixel mode and the sprite attribute determines the pattern table
                //     // Because the sprite is double the height it means we have half the sprites available
                //     if flip_vertically {
                //         let cell = if (self.scanline - (oam_entry.y as i16)) < 8 {
                //             // Top half
                //             (((oam_entry.tile_id as u16) & 0xFE) + 1) << 4
                //         } else {
                //             // Bottom half
                //             ((oam_entry.tile_id as u16) & 0xFE) << 4
                //         };

                //         sprite_pattern_address_low = 
                //             (((oam_entry.tile_id & 0x01) as u16) << 12) |
                //             cell |
                //             (row & 0x07);
                //     } else {
                //         let cell = if (self.scanline - (oam_entry.y as i16)) < 8 {
                //             // Top half
                //             ((oam_entry.tile_id as u16) & 0xFE) << 4
                //         } else {
                //             // Bottom half
                //             (((oam_entry.tile_id as u16) & 0xFE) + 1) << 4
                //         };

                //         sprite_pattern_address_low = 
                //             (((oam_entry.tile_id & 0x01) as u16) << 12) |
                //             cell |
                //             (row & 0x07);
                //     }
                // }

                sprite_pattern_address_high = sprite_pattern_address_low + 8;
                sprite_pattern_bit_low = self.ppu_read(sprite_pattern_address_low, false);
                sprite_pattern_bit_high = self.ppu_read(sprite_pattern_address_high, false);

                if flip_horizontally {
                    sprite_pattern_bit_low = sprites::flip_byte_horizontally(sprite_pattern_bit_low);
                    sprite_pattern_bit_high = sprites::flip_byte_horizontally(sprite_pattern_bit_high);
                }

                self.sprite_shifter_pattern_low[i] = sprite_pattern_bit_low;
                self.sprite_shifter_pattern_high[i] = sprite_pattern_bit_high;
            }
        }
    }

    /// Check to see if each sprite should be rendered on the current scanline
    /// This is done checking the y coordinates of each sprite to the current visible scanline
    /// If it's greater
    fn evaluate_sprites(&mut self) {
        // Clear sprite scanline
        self.sprite_count = 0;
        for i in 0..self.sprite_scanline.len() {
            self.sprite_scanline[i] = 0xFF;
        }

        for i in 0..sprites::MAX_SPRITE_COUNT {
            self.sprite_shifter_pattern_low[i] = 0;
            self.sprite_shifter_pattern_high[i] = 0;
        }

        self.sprite_zero_hit_possible = false;
        let sprite_size = if self.get_control(Control2C02::SpriteSize) > 0 { 16 } else { 8 };
        let mut current_oam_entry: usize = 0;
        // You can only have 8 sprites on the screen
        while current_oam_entry < sprites::MAX_SPRITES && self.sprite_count <= sprites::MAX_SPRITE_COUNT {
            let asd = current_oam_entry * sprites::OAM_ENTRY_SIZE;
            let diff = (self.scanline as i16) - (self.oam[asd] as i16);
            if diff >= 0 && diff < sprite_size {
                if self.sprite_count < sprites::MAX_SPRITE_COUNT {
                    if current_oam_entry == 0 {
                        self.sprite_zero_hit_possible = true;
                    }

                    for i in 0..sprites::OAM_ENTRY_SIZE {
                        let sprite_index = (self.sprite_count * sprites::OAM_ENTRY_SIZE) + i;
                        let oam_index = asd + i;
                        self.sprite_scanline[sprite_index] = self.oam[oam_index];
                    }

                    self.sprite_count += 1;
                }
            }

            current_oam_entry += 1;
        }

        self.set_status(Status2C02::SpriteOverflow, self.sprite_count > 8);
    }

    fn render_pixel(&mut self) {
        let (bg_palette, bg_pixel) = self.get_background_pixel();
        let (fg_palette, fg_pixel, fg_priority_over_bg) = self.get_foreground_pixels();

        let mut pixel = 0x00;
        let mut palette = 0x00;

        if bg_pixel == 0 && fg_pixel == 0 {
            // They're both transparent so no one wins
            pixel = 0x00;
            palette = 0x00;
        } else if bg_pixel == 0 && fg_pixel > 0 {
            // Foreground wins since background is transparent and foreground isn't
            pixel = fg_pixel;
            palette = fg_palette;
        } else if bg_pixel > 0 && fg_pixel == 0 {
            // Background wins since foreground is transparent and foreground isn't
            pixel = bg_pixel;
            palette = bg_palette;
        } else if bg_pixel > 0 && fg_pixel > 0 {
            // Both background and foreground are visible
            // We then check the priority over background flag
            if fg_priority_over_bg {
                pixel = fg_pixel;
                palette = fg_palette;
            } else {
                pixel = bg_pixel;
                palette = bg_palette;
            }

            // If both background and foreground aren't transparent we then check sprite zero hit
            if self.sprite_zero_hit_possible && self.sprite_zero_being_rendered {
                if self.get_mask(Mask2C02::RenderBackground) && self.get_mask(Mask2C02::RenderSprite) {
                    // The left edge of the screen has specific switches to control
                    // its appearance. This is used to smooth inconsistencies when
                    // scrolling (since sprites x coord must be >= 0)
                    let lower_cycle = if !(self.get_mask(Mask2C02::RenderBackgroundLeft) || self.get_mask(Mask2C02::RenderSpriteLeft)) {
                        9
                    } else {
                        1
                    };

                    if self.cycle >= lower_cycle && self.cycle <= MAX_VISIBLE_CLOCK_CYCLE {
                        self.set_status(Status2C02::SpriteZeroHit, true);
                    }
                }
            }
        }

        let color = self.get_color_from_palette(palette as u16, pixel as u16);
        if self.cycle > 0 {
            let x = self.cycle - 1;
            self.frame.set_pixel((x) as i32, self.scanline as i32, color);
        }
    }

    fn get_background_pixel(&mut self) -> (u8, u8) {
        let mut bg_palette = 0x00;
        let mut bg_pixel = 0x00;
        if self.get_mask(Mask2C02::RenderBackground) {
            let shift_register_bit = 0x8000 >> self.fine_x_scroll;
            let pixel_plane_0 = if (self.bg_shifter_pattern_low & shift_register_bit) > 0 { 1 } else { 0 };
            let pixel_plane_1 = if (self.bg_shifter_pattern_high & shift_register_bit) > 0 { 1 } else { 0 };

            bg_pixel = (pixel_plane_1 << 1) | pixel_plane_0;

            let bg_palette_0 = if (self.bg_shifter_attribute_low & shift_register_bit) > 0 { 1 } else { 0 };
            let bg_palette_1 = if (self.bg_shifter_attribute_high & shift_register_bit) > 0 { 1 } else { 0 };

            bg_palette = (bg_palette_1 << 1) | bg_palette_0;
        }

        (bg_palette, bg_pixel)
    }

    fn get_foreground_pixels(&mut self) -> (u8, u8, bool) {
        let mut fg_pixel = 0x00;
        let mut fg_palette = 0x00;
        let mut fg_priority_over_background = false;

        if self.get_mask(Mask2C02::RenderSprite) {
            self.sprite_zero_being_rendered = false;

            for i in 0..self.sprite_count {
                let oam_entry = sprites::get_object_attribute_entry(&self.sprite_scanline, i * sprites::OAM_ENTRY_SIZE);
                if oam_entry.x == 0 {
                    let pixel_plane_0 = if (self.sprite_shifter_pattern_low[i] & 0x80) > 0 { 1 } else { 0 };
                    let pixel_plane_1 = if (self.sprite_shifter_pattern_high[i] & 0x80) > 0 { 1 } else { 0 };
                    fg_pixel = (pixel_plane_1 << 1) | pixel_plane_0;

                    let palette_plane_0 = oam_entry.get_oam_attribute(sprites::OamAttribute::Palette0) << 0;
                    let palette_plane_1 = oam_entry.get_oam_attribute(sprites::OamAttribute::Palette1) << 1;
                    fg_palette = (palette_plane_1 | palette_plane_0) + 4; // The foreground palettes were the last 4 (4-7)
                    fg_priority_over_background = oam_entry.get_oam_attribute(sprites::OamAttribute::Priority) == 0;

                    // We know the sprites are in priority order(earliest address is higher priority)
                    // We also know that if a pixel is 0 it is transparent
                    // Therefore the first pixel that's not transparent is the highest priority pixel so break out
                    if fg_pixel != 0 {
                        if i == 0 { // If it's in 0 of our sprite scanline then it's a candidate for sprite 0
                            self.sprite_zero_being_rendered = true;
                        }

                        break;
                    }
                }
            }
        }

        (fg_palette, fg_pixel, fg_priority_over_background)
    }

    fn increment_x(&mut self) {
        if self.is_rendering_enabled() {
            if (self.current_vram_address & 0x001F) == 31 {
                self.current_vram_address &= !0x001F; // Set coarse x = 0
                self.current_vram_address ^= 0x0400; // Switch horizontal table
            } else {
                self.current_vram_address += 1; // Increment coarse x
            }
        }
    }

    fn increment_y(&mut self) {
        if self.is_rendering_enabled() {
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
    }

    fn load_shifters(&mut self) {
        self.bg_shifter_pattern_low = (self.bg_shifter_pattern_low & 0xFF00) | (self.bg_next_tile_lsb as u16);
        self.bg_shifter_pattern_high = (self.bg_shifter_pattern_high & 0xFF00) | (self.bg_next_tile_msb as u16);

        // Attribute bits don't change per pixel, but for every tile(8 pixels)
        // We then inflate the bottom and top bit to 8 bits
        self.bg_shifter_attribute_low = (self.bg_shifter_attribute_low & 0xFF00) | (if (self.bg_next_tile_attribute & 0b01) > 0 { 0xFF } else { 0x00 });
        self.bg_shifter_attribute_high = (self.bg_shifter_attribute_high & 0xFF00) | (if (self.bg_next_tile_attribute & 0b10) > 0 { 0xFF } else { 0x00 });
    }

    fn update_shifters(&mut self) {
        if self.get_mask(Mask2C02::RenderBackground) {
            self.bg_shifter_pattern_low <<= 1;
            self.bg_shifter_pattern_high <<= 1;
            self.bg_shifter_attribute_low <<= 1;
            self.bg_shifter_attribute_high <<= 1;
        }

        if self.get_mask(Mask2C02::RenderSprite) && self.cycle >= 1 && self.cycle <= MAX_VISIBLE_CLOCK_CYCLE {
            for i in 0..self.sprite_count {
                // First thing that needs to be done is decrement the x coordinate or else we'll shift everything off the screen
                let x_index = (i * sprites::OAM_ENTRY_SIZE) + 3;
                if self.sprite_scanline[x_index] > 0 {
                    self.sprite_scanline[x_index] -= 1;
                } else {
                    self.sprite_shifter_pattern_low[i] <<= 1;
                    self.sprite_shifter_pattern_high[i] <<= 1;
                }
            }
        }
    }

    fn transfer_x_address(&mut self) {
        if self.is_rendering_enabled() {
            // If rendering is enabled, the PPU copies all bits related to horizontal position from t to v:
            // v: ....F.. ...EDCBA = t: ....F.....EDCBA
            let fedcba = self.temp_vram_address & 0x41F;
            self.set_current_address(ScrollAddress::CoarseX0, ((fedcba >> 0) & 0x01) > 0);
            self.set_current_address(ScrollAddress::CoarseX1, ((fedcba >> 1) & 0x01) > 0);
            self.set_current_address(ScrollAddress::CoarseX2, ((fedcba >> 2) & 0x01) > 0);
            self.set_current_address(ScrollAddress::CoarseX3, ((fedcba >> 3) & 0x01) > 0);
            self.set_current_address(ScrollAddress::CoarseX4, ((fedcba >> 4) & 0x01) > 0);
            self.set_current_address(ScrollAddress::NameTableSelectX, ((fedcba >> 10) & 0x01) > 0);
        }
    }

    fn transfer_y_address(&mut self) {
        if self.is_rendering_enabled() {
            // If rendering is enabled, at the end of vblank, shortly after the horizontal bits are copied from t to v at dot 257, 
            // the PPU will repeatedly copy the vertical bits from t to v from dots 280 to 304, completing the full initialization of v from t:
            // v: IHGF.EDCBA..... = t: IHGF.ED CBA.....
            let ihgfedcba = self.temp_vram_address & 0x7BE0;
            self.set_current_address(ScrollAddress::CoarseY0, ((ihgfedcba >> 5) & 0x01) > 0);
            self.set_current_address(ScrollAddress::CoarseY1, ((ihgfedcba >> 6) & 0x01) > 0);
            self.set_current_address(ScrollAddress::CoarseY2, ((ihgfedcba >> 7) & 0x01) > 0);
            self.set_current_address(ScrollAddress::CoarseY3, ((ihgfedcba >> 8) & 0x01) > 0);
            self.set_current_address(ScrollAddress::CoarseY4, ((ihgfedcba >> 9) & 0x01) > 0);
            self.set_current_address(ScrollAddress::NameTableSelectY, ((ihgfedcba >> 11) & 0x01) > 0);
            self.set_current_address(ScrollAddress::FineYScroll0, ((ihgfedcba >> 12) & 0x01) > 0);
            self.set_current_address(ScrollAddress::FineYScroll1, ((ihgfedcba >> 13) & 0x01) > 0);
            self.set_current_address(ScrollAddress::FineYScroll2, ((ihgfedcba >> 14) & 0x01) > 0);
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
        let masked_address = address & 0x0FFF;
        let address_offset = (masked_address & 0x03FF) as usize; // Offset by size of name table(1023)

        match self.cartridge {
            Some(ref mut c) => {
                match c.borrow_mut().mirror {
                    cartridge::Mirror::Vertical => {
                        if masked_address <= 0x03FF {
                            data = self.name_table[0][address_offset];
                        } else if masked_address >= 0x0400 && masked_address <= 0x07FF {
                            data = self.name_table[1][address_offset];
                        } else if masked_address >= 0x800 && masked_address <= 0x0BFF {
                            data = self.name_table[0][address_offset];
                        } else if masked_address >= 0x0C00 && masked_address <= 0x0FFF {
                            data = self.name_table[1][address_offset];
                        }
                    },
                    cartridge::Mirror::Horizontal => {
                        if masked_address <= 0x03FF {
                            data = self.name_table[0][address_offset];
                        } else if masked_address >= 0x0400 && masked_address <= 0x07FF {
                            data = self.name_table[0][address_offset];
                        } else if masked_address >= 0x800 && masked_address <= 0x0BFF {
                            data = self.name_table[1][address_offset];
                        } else if masked_address >= 0x0C00 && masked_address <= 0x0FFF {
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

    fn get_color_from_palette(&mut self, palette_id: u16, pixel_id: u16) -> Color {
        let address = addresses::PALETTE_ADDRESS_LOWER + (palette_id * 4) + pixel_id;
        let color_index = self.ppu_read(address, false) & 0x3F; // Make sure we don't go out of bounds
        self.colors[color_index as usize]
    }

    fn get_pattern_address(&mut self, offset: u16) -> u16 {
        let upper = (self.get_control(Control2C02::BackgroundTableAddress) as u16) << 12;
        let middle = (self.bg_next_tile_id as u16) << 4;
        let lower = self.get_fine_y();
        let address = upper + middle + lower + offset;

        address
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
        self.mask & (mask as u8) > 0
    }

    fn set_status(&mut self, status: Status2C02, value: bool) {
        if value {
            self.status = self.status | (status as u8);
        } else {
            self.status = self.status & !(status as u8);
        }
    }

    fn set_current_address(&mut self, scroll: ScrollAddress, value: bool) {
        if value {
            self.current_vram_address |= scroll as u16;
        } else {
            self.current_vram_address &= !(scroll as u16);
        }
    }

    fn get_current_address(&mut self, scroll: ScrollAddress) -> u16 {
        if self.current_vram_address & (scroll as u16) > 0 {
            1
        } else {
            0
        }
    }

    fn set_temp_address(&mut self, scroll: ScrollAddress, value: bool) {
        if value {
            self.temp_vram_address |= scroll as u16;
        } else {
            self.temp_vram_address &= !(scroll as u16);
        }
    }

    fn get_coarse_x(&mut self) -> u16 {
        let coarse_x_0 = self.get_current_address(ScrollAddress::CoarseX0) << 0;
        let coarse_x_1 = self.get_current_address(ScrollAddress::CoarseX1) << 1;
        let coarse_x_2 = self.get_current_address(ScrollAddress::CoarseX2) << 2;
        let coarse_x_3 = self.get_current_address(ScrollAddress::CoarseX3) << 3;
        let coarse_x_4 = self.get_current_address(ScrollAddress::CoarseX4) << 4;

        coarse_x_0 + coarse_x_1 + coarse_x_2 + coarse_x_3 + coarse_x_4
    }

    fn get_coarse_y(&mut self) -> u16 {
        let coarse_y_0 = self.get_current_address(ScrollAddress::CoarseY0) << 0;
        let coarse_y_1 = self.get_current_address(ScrollAddress::CoarseY1) << 1;
        let coarse_y_2 = self.get_current_address(ScrollAddress::CoarseY2) << 2;
        let coarse_y_3 = self.get_current_address(ScrollAddress::CoarseY3) << 3;
        let coarse_y_4 = self.get_current_address(ScrollAddress::CoarseY4) << 4;

        coarse_y_0 + coarse_y_1 + coarse_y_2 + coarse_y_3 + coarse_y_4
    }

    fn get_fine_y(&mut self) -> u16 {
        let fine_y_0 = self.get_current_address(ScrollAddress::FineYScroll0) << 0;
        let fine_y_1 = self.get_current_address(ScrollAddress::FineYScroll1) << 1;
        let fine_y_2 = self.get_current_address(ScrollAddress::FineYScroll2) << 2;

        fine_y_0 + fine_y_1 + fine_y_2
    }

    fn is_rendering_enabled(&mut self) -> bool {
        self.get_mask(Mask2C02::RenderBackground) || self.get_mask(Mask2C02::RenderSprite)
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
    NameTableSelectX = (1 << 10),
    NameTableSelectY = (1 << 11),
    FineYScroll0 = (1 << 12),
    FineYScroll1 = (1 << 13),
    FineYScroll2 = (1 << 14)
}

fn get_colors() -> Vec<Color> {
    vec![
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

fn initialize_oam() -> Vec<u8> {
    let capacity = sprites::OAM_ENTRY_SIZE * sprites::MAX_SPRITES;
    let mut vec: Vec<u8> = Vec::with_capacity(capacity as usize);

    for _ in 0..capacity {
        vec.push(0);
    }

    vec
}