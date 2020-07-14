#[derive(Debug)]
pub struct ObjectAttributeEntry {
    pub y: u8,
    pub tile_id: u8,
    pub attribute: OamAttribute,
    pub x: u8
}

bitfield! {
    pub struct OamAttribute(u8);
    impl Debug;

    pub palette_0, _: 0; // Palette (4 to 7) of sprite
    pub palette_1, _: 1; // Palette (4 to 7) of sprite
    pub palette, _: 1, 0; // Full palette (4 to 7) of sprite
    pub unimplemented_0, _: 2; // Unimplemented
    pub unimplemented_1, _: 3; // Unimplemented
    pub unimplemented_2, _: 4; // Unimplemented
    pub priority, _: 5; // Priority (0: in front of background; 1: behind background)
    pub flip_horizontally, _: 6; // Flip sprite horizontally
    pub flip_vertically, _: 7; // Flip sprite vertically
}