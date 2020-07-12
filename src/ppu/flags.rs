bitfield!{
    pub struct Status(u8);
    impl Debug;
    // The fields default to u16
    pub sprite_overflow, set_sprite_overflow: 5;
    pub sprite_zero_hit, set_sprite_zero_hit: 6;
    pub vertical_blank, set_vertical_blank: 7;
    pub get, _: 7, 0;
}

/* 
SpriteOverflow = (1 << 5),
    SpriteZeroHit = (1 << 6),
    VerticalBlank = (1 << 7)
*/