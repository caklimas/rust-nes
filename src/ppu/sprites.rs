use super::oam;

pub const OAM_ENTRY_SIZE: usize = 4;
pub const MAX_SPRITES: usize = 64;
pub const MAX_SPRITE_COUNT: usize = 64;

#[derive(Debug, Default)]
pub struct Sprite {
    pub sprite_scanline: Vec<u8>,
    pub count: usize,
    pub shifter_pattern_low: Vec<u8>,
    pub shifter_pattern_high: Vec<u8>,
    pub zero_hit_possible: bool,
    pub zero_being_rendered: bool
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            sprite_scanline: vec![0; MAX_SPRITE_COUNT * OAM_ENTRY_SIZE], // 8 sprites times size of an entry
            count: 0,
            shifter_pattern_low: vec![0; MAX_SPRITE_COUNT],
            shifter_pattern_high: vec![0; MAX_SPRITE_COUNT],
            zero_hit_possible: false,
            zero_being_rendered: false
        }
    }

    pub fn reset(&mut self) {
        for i in 0..MAX_SPRITE_COUNT {
            self.shifter_pattern_low[i] = 0;
            self.shifter_pattern_high[i] = 0;
        }
    }

    pub fn update_shifters(&mut self) {
        for i in 0..self.count {
            // First thing that needs to be done is decrement the x coordinate or else we'll shift everything off the screen
            let x_index = (i * OAM_ENTRY_SIZE) + 3;
            if self.sprite_scanline[x_index] > 0 {
                self.sprite_scanline[x_index] -= 1;
            } else {
                self.shifter_pattern_low[i] <<= 1;
                self.shifter_pattern_high[i] <<= 1;
            }
        }
    }

    pub fn get_pixel(&mut self) -> (u8, u8, bool) {
        let mut pixel = 0x00;
        let mut palette = 0x00;
        let mut priority_over_background = false;
        self.zero_being_rendered = false;

        for i in 0..self.count {
            let oam_entry = self.get_object_attribute_entry(i * OAM_ENTRY_SIZE);
            if oam_entry.x == 0 {
                let pixel_plane_0 = if (self.shifter_pattern_low[i] & 0x80) > 0 { 1 } else { 0 };
                let pixel_plane_1 = if (self.shifter_pattern_high[i] & 0x80) > 0 { 1 } else { 0 };
                pixel = (pixel_plane_1 << 1) | pixel_plane_0;

                palette = oam_entry.attribute.palette() + 4; // The foreground palettes were the last 4 (4-7)
                priority_over_background = !oam_entry.attribute.priority();

                // We know the sprites are in priority order(earliest address is higher priority)
                // We also know that if a pixel is 0 it is transparent
                // Therefore the first pixel that's not transparent is the highest priority pixel so break out
                if pixel != 0 {
                    if i == 0 { // If it's in 0 of our sprite scanline then it's a candidate for sprite 0
                        self.zero_being_rendered = true;
                    }

                    break;
                }
            }
        }

        (palette, pixel, priority_over_background)
    }

    pub fn get_pattern_address(&mut self, index: usize, sprite_mode: bool, sprite_table_address: u16, scanline: i16) -> (u16, bool) {
        let oam_entry = self.get_object_attribute_entry(index * OAM_ENTRY_SIZE);
        let mut sprite_pattern_bit_low: u8;
        let mut sprite_pattern_bit_high: u8;
        let sprite_pattern_address_low: u16;

        let pattern_table = if !sprite_mode { 
            sprite_table_address << 12 
        } else { 
            ((oam_entry.tile_id & 0x01) as u16) << 12
        };
        
        if !sprite_mode {
            let cell = (oam_entry.tile_id as u16) << 4;
            let row = if !oam_entry.attribute.flip_vertically() { 
                (scanline - (oam_entry.y as i16)) as u16
            } else { 
                7 - ((scanline - (oam_entry.y as i16)) as u16)
            };

            sprite_pattern_address_low = pattern_table | cell | row;
        } else {
            let row = if !oam_entry.attribute.flip_vertically() {
                ((scanline - (oam_entry.y as i16)) as u16 ) & 0x07
            } else {
                (7 - (scanline - (oam_entry.y as i16)) as u16 ) & 0x07
            };

            if !oam_entry.attribute.flip_vertically() {
                if scanline - (oam_entry.y as i16) < 8 {
                    sprite_pattern_address_low =
                        pattern_table |
                        (((oam_entry.tile_id & 0xFE) as u16) << 4) |
                        row;
                } else {
                    sprite_pattern_address_low =
                        pattern_table |
                        ((((oam_entry.tile_id & 0xFE) + 1) as u16) << 4) |
                        row;
                }
            } else {
                if scanline - (oam_entry.y as i16) < 8 {
                    sprite_pattern_address_low =
                        pattern_table |
                        ((((oam_entry.tile_id & 0xFE) + 1) as u16) << 4) |
                        row;
                } else {
                    sprite_pattern_address_low =
                        pattern_table |
                        (((oam_entry.tile_id & 0xFE) as u16) << 4) |
                        row;
                }
            }
        }

        (sprite_pattern_address_low, oam_entry.attribute.flip_horizontally())
    }

    fn get_object_attribute_entry(&mut self, index: usize) -> oam::ObjectAttributeEntry {
        oam::ObjectAttributeEntry {
            y: self.sprite_scanline[index + 0],
            tile_id: self.sprite_scanline[index + 1],
            attribute: oam::OamAttribute(self.sprite_scanline[index + 2]),
            x: self.sprite_scanline[index + 3]
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct DirectMemoryAccess {
    pub page: u8,
    pub address: u8,
    pub data: u8
}

/// Flipping a byte horizontally
/// Ex: 11100000 becomes 00000111
/// https://stackoverflow.com/a/2602885
pub fn flip_byte_horizontally(byte: u8) -> u8 {
    let mut flipped_byte = byte;
    flipped_byte = (flipped_byte & 0xF0) >> 4 | (flipped_byte & 0x0F) << 4;
    flipped_byte = (flipped_byte & 0xCC) >> 2 | (flipped_byte & 0x33) << 2;
    flipped_byte = (flipped_byte & 0xAA) >> 1 | (flipped_byte & 0x55) << 1;

    flipped_byte
}