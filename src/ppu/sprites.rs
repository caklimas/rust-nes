pub const OAM_ENTRY_SIZE: usize = 4;
pub const MAX_SPRITES: usize = 64;
pub const MAX_SPRITE_COUNT: usize = 64;

#[derive(Debug, Default)]
pub struct Sprite {
    pub scanline: Vec<u8>,
    pub count: usize,
    pub shifter_pattern_low: Vec<u8>,
    pub shifter_pattern_high: Vec<u8>,
    pub zero_hit_possible: bool,
    pub zero_being_rendered: bool
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            scanline: vec![0; MAX_SPRITE_COUNT * OAM_ENTRY_SIZE], // 8 sprites times size of an entry
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
            if self.scanline[x_index] > 0 {
                self.scanline[x_index] -= 1;
            } else {
                self.shifter_pattern_low[i] <<= 1;
                self.shifter_pattern_high[i] <<= 1;
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct DirectMemoryAccess {
    pub page: u8,
    pub address: u8,
    pub data: u8
}

pub fn get_object_attribute_entry(oam: &Vec<u8>, index: usize) -> ObjectAttributeEntry {
    ObjectAttributeEntry {
        y: oam[index + 0],
        tile_id: oam[index + 1],
        attribute: oam[index + 2],
        x: oam[index + 3]
    }
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

#[derive(Debug)]
pub struct ObjectAttributeEntry {
    pub y: u8,
    pub tile_id: u8,
    pub attribute: u8,
    pub x: u8
}

impl ObjectAttributeEntry {
    pub fn get_oam_attribute(&self, attribute: OamAttribute) -> u8 {
        let value = self.attribute & (attribute as u8);
        if value > 0 {
            1
        } else {
            0
        }
    }
}

pub enum OamAttribute {
    Palette0 = (1 << 0), // Palette (4 to 7) of sprite
    Palette1 = (1 << 1), // Palette (4 to 7) of sprite
    Unimplemented0 = (1 << 2), // Unimplemented
    Unimplemented1 = (1 << 3), // Unimplemented
    Unimplemented2 = (1 << 4), // Unimplemented
    Priority = (1 << 5), // Priority (0: in front of background; 1: behind background)
    FlipHorizontally = (1 << 6), // Flip sprite horizontally
    FlipVertically = (1 << 7) // Flip sprite vertically
}