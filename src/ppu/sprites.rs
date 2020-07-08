use crate::ppu::ppu;

pub const OAM_ENTRY_SIZE: usize = 4;
pub const MAX_SPRITES: usize = 64;
pub const MAX_SPRITE_COUNT: usize = 16;

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