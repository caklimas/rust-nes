use std::rc::Rc;
use std::cell::RefCell;

use crate::addresses::cpu::*;
use crate::addresses::ppu::*;
use super::background;
use crate::cartridge::cartridge;
use crate::cartridge::mirror::Mirror;
use super::flags;
use super::frame;
use crate::memory_sizes;
use super::oam;
use super::sprites;

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
    pub frame: frame::Frame,
    pub oam: oam::ObjectAttributeMemory,
    scanline: i16,
    cycle: u16,
    status: super::flags::Status,
    control: flags::Control,
    mask: super::flags::Mask,
    address_latch: bool,
    ppu_data_buffer: u8,
    current_vram_address: flags::ScrollAddress,
    temp_vram_address: flags::ScrollAddress,
    fine_x_scroll: u8,
    background: background::Background,
    sprite: sprites::Sprite
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
            frame: frame::Frame::new(),
            oam: oam::ObjectAttributeMemory::new(),
            status: flags::Status(0),
            control: flags::Control(0),
            mask: super::flags::Mask(0),
            address_latch: false,
            ppu_data_buffer: 0,
            current_vram_address: flags::ScrollAddress(0),
            temp_vram_address: flags::ScrollAddress(0),
            fine_x_scroll: 0,
            background: Default::default(),
            sprite: sprites::Sprite::new()
        }
    }
   
    pub fn clock(&mut self) -> bool {
        if self.scanline >= -1 && self.scanline <= MAX_VISIBLE_SCANLINE {
            // Skipped on BG+odd
            if self.scanline == 0 && self.cycle == 0 {
                self.cycle = 1;
            }

            if self.scanline == -1 && self.cycle == 1 {
                self.status.set_vertical_blank(false);
                self.status.set_sprite_overflow(false);
                self.status.set_sprite_zero_hit(false);

                self.sprite.reset();
            }

            self.render_background();
            self.render_foreground();
        }

        if self.scanline == 240 {
            // Post render scanline does nothing
        }

        if self.scanline >= 241 && self.scanline < 261 {
            if self.scanline == 241 && self.cycle == 1 {
                self.status.set_vertical_blank(true);
                if self.control.generate_nmi() {
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
            }
        }

        self.cycle == MAX_VISIBLE_CLOCK_CYCLE && self.scanline == 240
    }

    /// Read from the Main Bus
    pub fn read(&mut self, address: u16) -> u8 {
        let masked_address = address & PPU_ADDRESS_RANGE;
        let mut data: u8 = 0;
        match masked_address {
            CONTROL => (), // Can't be read
            MASK => (), // Can't be read
            STATUS => {
                data = self.status.get() | (self.ppu_data_buffer & 0x1F);
                self.status.set_vertical_blank(false);
                self.address_latch = false;
            },
            OAM_ADDRESS => (),
            OAM_DATA => {
                data = self.oam.memory[self.oam.address as usize];
            },
            SCROLL => (),
            PPU_ADDRESS => (),
            PPU_DATA => {
                data = self.ppu_data_buffer;
                self.ppu_data_buffer = self.ppu_read(self.current_vram_address.get());

                if self.current_vram_address.get() >= PALETTE_ADDRESS_LOWER {
                    data = self.ppu_data_buffer;
                }

                self.current_vram_address.increment(self.control.get_increment_amount());
            },
            _ => ()
        };

        data
    }

    /// Write to the Main Bus
    pub fn write(&mut self, address: u16, data: u8) {
        let masked_address = address & PPU_ADDRESS_RANGE; 
        match masked_address {
            CONTROL => {
                self.control.set(data);

                // t: ...BA.......... = d: ......BA
                let ba = data & 0b11;
                self.temp_vram_address.set_name_table(ba);
            },
            MASK => {
                self.mask.set(data);
            },
            STATUS => (),
            OAM_ADDRESS => {
                self.oam.address = data;
            },
            OAM_DATA => {
                self.oam.memory[self.oam.address as usize] = data;
                self.oam.address = self.oam.address.wrapping_add(1);
            },
            SCROLL => {
                if !self.address_latch {
                    // t: ....... ...HGFED = d: HGFED...
                    // x:              CBA = d: .....CBA
                    // w:                  = 1
                    let hgfed = (data & 0b11111000) >> 3;
                    self.temp_vram_address.set_coarse_x(hgfed);
                    self.fine_x_scroll = data & 0b111;
                    self.address_latch = true;
                } else {
                    // t: CBA..HGFED..... = d: HGFEDCBA
                    // w:                  = 0
                    let cba = data & 0b111;
                    let hgfed = (data & 0b11111000) >> 3;
                    self.temp_vram_address.set_coarse_y(hgfed);
                    self.temp_vram_address.set_fine_y(cba);

                    self.address_latch = false;
                }
            },
            PPU_ADDRESS => {
                if !self.address_latch {
                    // t: .FEDCBA........ = d: ..FEDCBA
                    // t: X.............. = 0
                    // w:                  = 1
                    let fedbca = data & 0b00111111;
                    self.temp_vram_address.set_high_byte(fedbca);
                    self.address_latch = true;
                } else {
                    // t: ....... HGFEDCBA = d: HGFEDCBA
                    // v                   = t
                    // w:                  = 0
                    self.temp_vram_address.set_low_byte(data);

                    self.current_vram_address.set(self.temp_vram_address.get());
                    self.address_latch = false;
                }
            },
            PPU_DATA => {
                self.ppu_write(self.current_vram_address.get(), data);
                self.current_vram_address.increment(self.control.get_increment_amount());
            },
            _ => ()
        };
    }

    /// Read from the PPU Bus
    fn ppu_read(&mut self, address: u16) -> u8 {
        let mut data: u8 = 0;
        let ppu_address = address & PPU_ADDRESS_END;

        match self.cartridge {
            Some(ref mut c) => {
                if c.borrow_mut().ppu_read(ppu_address, &mut data) {
                    return data;
                }
            },
            None => ()
        };

        if ppu_address <= PATTERN_ADDRESS_UPPER {
            data = self.read_pattern_table_data(ppu_address);
        } else if ppu_address >= NAME_TABLE_ADDRESS_LOWER && ppu_address <= NAME_TABLE_ADDRESS_UPPER {
            data = self.read_name_table_data(ppu_address);
        } else if ppu_address >= PALETTE_ADDRESS_LOWER && ppu_address <= PALETTE_ADDRESS_UPPER {
            data = self.read_palette_table_data(ppu_address);
        }

        data
    }

    /// Write to the PPU Bus
    fn ppu_write(&mut self, address: u16, data: u8) {
        let ppu_address = address & PPU_ADDRESS_END;
        match self.cartridge {
            Some(ref mut c) => {
                if c.borrow_mut().ppu_write(ppu_address, data) {
                    return;
                }
            },
            None => ()
        };

        if ppu_address <= PATTERN_ADDRESS_UPPER {
            self.write_pattern_table_data(ppu_address, data);
        } else if ppu_address >= NAME_TABLE_ADDRESS_LOWER && ppu_address <= NAME_TABLE_ADDRESS_UPPER {
            self.write_name_table_data(ppu_address, data);
        } else if ppu_address >= PALETTE_ADDRESS_LOWER && ppu_address <= PALETTE_ADDRESS_UPPER {
            self.write_palette_table_data(ppu_address, data);
        }
    }

    fn render_background(&mut self) {
        if (self.cycle >= 2 && self.cycle <= MAX_VISIBLE_CLOCK_CYCLE) || (self.cycle >= 321 && self.cycle < 338) {
            self.update_shifters();

            let sub_cycle = (self.cycle - 1) % 8;
            if sub_cycle == 0 {
                self.background.load_shifters();
                let name_table_address = self.current_vram_address.name_table_address();
                self.background.next_tile_id = self.ppu_read(name_table_address);
            } else if sub_cycle == 2 {
                let attribute_table_address = self.current_vram_address.attribute_table_address();
                self.background.next_tile_attribute = self.ppu_read(attribute_table_address);

                // Since there are only 4 palettes for the background tiles, we only need 2 bits to select a palette(2 bits range is 0-3)
                // We get a byte of data we can split that byte up into 4 sets of 2 bits.
                // One attribute byte covers a block of data (4x4 tiles) so we can apply each set of 2 bits to one quadrant of the block
                // Bits 7,6 => bottom right, Bits 5,4 => bottom left, Bits 3,2 => top right, Bits 1,0 => top left
                // If coarse y % 4 < 2 then it is in the top half
                // If coarse x % 4 < 2 then it is in the left half
                // Knowing this and that we want the last two bits to be the palette selected so we shift accordingly.
                if self.current_vram_address.coarse_y() & 0x02 > 0 {
                    self.background.next_tile_attribute >>= 4; // Use bits 7,6 or 5,4
                }

                if self.current_vram_address.coarse_x() & 0x02 > 0 {
                    self.background.next_tile_attribute >>= 2; // USe bits 7,6 or 3,2
                }

                self.background.next_tile_attribute &= 0x03;
            } else if sub_cycle == 4 {
                let pattern_address = self.get_pattern_address(0);
                self.background.next_tile_lsb = self.ppu_read(pattern_address);
            } else if sub_cycle == 6 {
                let pattern_address = self.get_pattern_address(8);
                self.background.next_tile_msb = self.ppu_read(pattern_address);
            } else if sub_cycle == 7 {
                if self.mask.is_rendering_enabled() {
                    self.current_vram_address.increment_x();
                }
            }
        }

        if self.cycle == 256 {
            // If rendering is enabled, the PPU increments the vertical position in v.
            // The effective Y scroll coordinate is incremented, which is a complex operation that will correctly skip the attribute table memory regions,
            // and wrap to the next nametable appropriately.
            if self.mask.is_rendering_enabled() {
                self.current_vram_address.increment_y();
            }
        }
        
        if self.cycle == MAX_VISIBLE_CLOCK_CYCLE {
            self.background.load_shifters();
            self.transfer_x_address();
        }

        if self.scanline == -1 && self.cycle >= 280 && self.cycle <= 304 {
            self.transfer_y_address();
        }

        // Useless read of the tile id at the end of the scanline
        if self.cycle == 338 || self.cycle == 340 {
            let name_table_address = self.current_vram_address.name_table_address();
            self.background.next_tile_id = self.ppu_read(name_table_address);
        }
    }

    fn render_foreground(&mut self) {
        // This isn't exactly how the NES does foreground rendering, however it gets there most of the way
        if self.cycle == MAX_VISIBLE_CLOCK_CYCLE && self.scanline >= 0 {
            self.evaluate_sprites();
        }

        if self.cycle == MAX_CLOCK_CYCLE - 1 {
            for i in 0..self.sprite.count {
                let (sprite_pattern_address_low, flip_horizontally) = self.sprite.get_pattern_address(
                    i,
                    self.control.sprite_size(), 
                    self.control.sprite_table_address() as u16,
                    self.scanline
                );

                let sprite_pattern_address_high = sprite_pattern_address_low + 8;
                let mut sprite_pattern_bit_low = self.ppu_read(sprite_pattern_address_low);
                let mut sprite_pattern_bit_high = self.ppu_read(sprite_pattern_address_high);

                if flip_horizontally {
                    sprite_pattern_bit_low = sprites::flip_byte_horizontally(sprite_pattern_bit_low);
                    sprite_pattern_bit_high = sprites::flip_byte_horizontally(sprite_pattern_bit_high);
                }

                self.sprite.shifter_pattern_low[i] = sprite_pattern_bit_low;
                self.sprite.shifter_pattern_high[i] = sprite_pattern_bit_high;
            }
        }
    }

    /// Check to see if each sprite should be rendered on the current scanline
    /// This is done checking the y coordinates of each sprite to the current visible scanline
    /// If it's greater than 8, then set sprite overflow
    fn evaluate_sprites(&mut self) {
        let sprite_size = self.control.get_sprite_size();
        self.sprite.evaluate_sprites(self.scanline, sprite_size.into(), &self.oam);
        self.status.set_sprite_overflow(self.sprite.count > 8);
    }

    fn render_pixel(&mut self) {
        let (bg_palette, bg_pixel) = self.get_background_pixel();
        let (fg_palette, fg_pixel, fg_priority_over_bg) = self.get_foreground_pixel();

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
            if self.sprite.zero_hit_possible && self.sprite.zero_being_rendered {
                if self.mask.render_background() && self.mask.render_sprite() {
                    // The left edge of the screen has specific switches to control
                    // its appearance. This is used to smooth inconsistencies when
                    // scrolling (since sprites x coord must be >= 0)
                    let lower_cycle = if !(self.mask.render_background_left() || self.mask.render_sprite_left()) {
                        9
                    } else {
                        1
                    };

                    if self.cycle >= lower_cycle && self.cycle <= MAX_VISIBLE_CLOCK_CYCLE {
                        self.status.set_sprite_zero_hit(true);
                    }
                }
            }
        }

        let color = self.get_color_from_palette(palette as u16, pixel as u16);
        if self.cycle > 0 {
            self.frame.set_pixel((self.cycle - 1) as usize, self.scanline as usize, color);
        }
    }

    fn get_background_pixel(&mut self) -> (u8, u8) {
        if !self.mask.render_background() {
            return (0, 0);
        }

        self.background.get_pixel(self.fine_x_scroll)
    }

    fn get_foreground_pixel(&mut self) -> (u8, u8, bool) {
        if !self.mask.render_sprite() {
            return (0, 0, false);
        }

        self.sprite.get_pixel()
    }

    fn update_shifters(&mut self) {
        if self.mask.render_background() {
            self.background.update_shifters();
        }

        if self.mask.render_sprite() && self.cycle >= 1 && self.cycle <= MAX_VISIBLE_CLOCK_CYCLE {
            self.sprite.update_shifters();
        }
    }

    fn transfer_x_address(&mut self) {
        if self.mask.is_rendering_enabled() {
            // If rendering is enabled, the PPU copies all bits related to horizontal position from t to v:
            // v: ....F.. ...EDCBA = t: ....F.....EDCBA
            self.current_vram_address.transfer_x_address(self.temp_vram_address);
        }
    }

    fn transfer_y_address(&mut self) {
        if self.mask.is_rendering_enabled() {
            // If rendering is enabled, at the end of vblank, shortly after the horizontal bits are copied from t to v at dot 257, 
            // the PPU will repeatedly copy the vertical bits from t to v from dots 280 to 304, completing the full initialization of v from t:
            // v: IHGF.EDCBA..... = t: IHGF.ED CBA.....
            self.current_vram_address.transfer_y_address(self.temp_vram_address);
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
                match c.borrow_mut().get_mirror() {
                    Mirror::Vertical => {
                        if masked_address <= 0x03FF {
                            data = self.name_table[0][address_offset];
                        } else if masked_address >= 0x0400 && masked_address <= 0x07FF {
                            data = self.name_table[1][address_offset];
                        } else if masked_address >= 0x0800 && masked_address <= 0x0BFF {
                            data = self.name_table[0][address_offset];
                        } else if masked_address >= 0x0C00 && masked_address <= 0x0FFF {
                            data = self.name_table[1][address_offset];
                        }
                    },
                    Mirror::Horizontal => {
                        if masked_address <= 0x07FF {
                            data = self.name_table[0][address_offset];
                        } else if masked_address >= 0x0800 && masked_address <= 0x0FFF {
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
                match c.borrow_mut().get_mirror() {
                    Mirror::Vertical => {
                        if masked_address <= 0x03FF {
                            self.name_table[0][address_offset] = data;
                        } else if masked_address >= 0x0400 && masked_address <= 0x07FF {
                            self.name_table[1][address_offset] = data;
                        } else if masked_address >= 0x0800 && masked_address <= 0x0BFF {
                            self.name_table[0][address_offset] = data;
                        } else if masked_address >= 0x0C00 && masked_address <= 0x0FFF {
                            self.name_table[1][address_offset] = data;
                        }
                    },
                    Mirror::Horizontal => {
                        if masked_address <= 0x07FF {
                            self.name_table[0][address_offset] = data;
                        } else if masked_address >= 0x0800 && masked_address <= 0x0FFF {
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

    fn get_color_from_palette(&mut self, palette_id: u16, pixel_id: u16) -> super::Color {
        let address = PALETTE_ADDRESS_LOWER + (palette_id * 4) + pixel_id;
        let color_index = self.ppu_read(address) & 0x3F; // Make sure we don't go out of bounds
        super::COLOR_RAM[color_index as usize]
    }

    fn get_pattern_address(&mut self, offset: u16) -> u16 {
        ((self.control.background_table_address() as u16) << 12) +
        ((self.background.next_tile_id as u16) << 4) +
        (self.current_vram_address.fine_y() as u16) +
        offset
    }
}