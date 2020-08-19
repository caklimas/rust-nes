pub mod background;
pub mod colors;
pub mod debug;
pub mod flags;
pub mod frame;
pub mod fps_limiter;
pub mod name_table;
pub mod oam;
pub mod palette_table;
pub mod pattern_table;
pub mod registers;
pub mod sprites;

use serde::{Serialize, Deserialize};
use std::rc::Rc;
use std::cell::RefCell;

use crate::addresses::cpu::*;
use crate::addresses::ppu::*;
use crate::cartridge;

const IRQ_CLOCK_CYCLE: u16 = 260;
const MAX_CLOCK_CYCLE: u16 = 341;
const MAX_SCANLINE: i16 = 261;
const MAX_VISIBLE_SCANLINE: i16 = 239;
const MAX_VISIBLE_CLOCK_CYCLE: u16 = 257;

#[derive(Serialize, Deserialize)]
pub struct Ppu2C02 {
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub nmi: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub frame: frame::Frame,
    pub oam: oam::ObjectAttributeMemory,
    address_latch: bool,
    background: background::Background,
    control: flags::Control,
    current_vram_address: flags::ScrollAddress,
    cycle: u16,
    fine_x_scroll: u8,
    mask: flags::Mask,
    name_table: name_table::NameTable, 
    pallete_table: palette_table::PaletteTable,
    pattern_table: pattern_table::PatternTable,
    ppu_data_buffer: u8,
    scanline: i16,
    sprite: sprites::Sprite,
    status: flags::Status,
    temp_vram_address: flags::ScrollAddress
}

impl Ppu2C02 {
    pub fn new() -> Self {
        Ppu2C02 {
            name_table: name_table::NameTable::new(),
            pallete_table: palette_table::PaletteTable::new(),
            pattern_table: pattern_table::PatternTable::new(),
            cartridge: None,
            nmi: false,
            scanline: 0,
            cycle: 0,
            frame: frame::Frame::new(),
            oam: oam::ObjectAttributeMemory::new(),
            status: flags::Status(0),
            control: flags::Control(0),
            mask: flags::Mask(0),
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

        if self.scanline == 241 && self.scanline < 261 && self.cycle == 1 {
            self.status.set_vertical_blank(true);
            if self.control.generate_nmi() {
                self.nmi = true;
            }
        }

        if self.scanline != -1 || self.scanline != 261 {
            self.render_pixel();
        }

        self.cycle += 1;

        // The IRQ counter should decrement on PPU cycle 260, right after the visible part of the target scanline has ended
        if self.mask.is_rendering_enabled() && self.cycle == IRQ_CLOCK_CYCLE && self.scanline < 240 {
            if let Some(ref mut c) = self.cartridge {
                if let Some(ref mut m) = c.borrow_mut().mapper {
                    m.irq_scanline();
                }
            }
        }

        if self.cycle >= MAX_CLOCK_CYCLE {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= MAX_SCANLINE {
                self.scanline = -1;
            }
        }

        self.cycle == MAX_VISIBLE_CLOCK_CYCLE && self.scanline == 240
    }

    /// Read from the PPU Bus
    fn ppu_read(&self, address: u16) -> u8 {
        let mut data: u8 = 0;
        let ppu_address = address & PPU_ADDRESS_END;

        if let Some(ref c) = self.cartridge {
            if c.borrow_mut().ppu_read(ppu_address, &mut data) {
                return data;
            }
        }

        if ppu_address <= PATTERN_ADDRESS_UPPER {
            data = self.pattern_table.read_data(ppu_address);
        } else if ppu_address >= NAME_TABLE_ADDRESS_LOWER && ppu_address <= NAME_TABLE_ADDRESS_UPPER {
            data = self.name_table.read_data(ppu_address, &self.cartridge);
        } else if ppu_address >= PALETTE_ADDRESS_LOWER && ppu_address <= PALETTE_ADDRESS_UPPER {
            data = self.pallete_table.read_data(ppu_address);
        }

        data
    }

    /// Write to the PPU Bus
    fn ppu_write(&mut self, address: u16, data: u8) {
        let ppu_address = address & PPU_ADDRESS_END;
        if let Some(ref mut c) = self.cartridge {
            if c.borrow_mut().ppu_write(ppu_address, data) {
                return;
            }
        }

        if ppu_address <= PATTERN_ADDRESS_UPPER {
            self.pattern_table.write_data(ppu_address, data);
        } else if ppu_address >= NAME_TABLE_ADDRESS_LOWER && ppu_address <= NAME_TABLE_ADDRESS_UPPER {
            self.name_table.write_data(ppu_address, &self.cartridge, data);
        } else if ppu_address >= PALETTE_ADDRESS_LOWER && ppu_address <= PALETTE_ADDRESS_UPPER {
            self.pallete_table.write_data(ppu_address, data);
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

    pub fn get_color_from_palette(&self, palette_id: u16, pixel_id: u16) -> colors::Color {
        let address = PALETTE_ADDRESS_LOWER + (palette_id * 4) + pixel_id;
        let color_index = self.ppu_read(address) & 0x3F; // Make sure we don't go out of bounds
        colors::COLOR_RAM[color_index as usize]
    }

    fn get_pattern_address(&mut self, offset: u16) -> u16 {
        ((self.control.background_table_address() as u16) << 12) +
        ((self.background.next_tile_id as u16) << 4) +
        (self.current_vram_address.fine_y() as u16) +
        offset
    }
}