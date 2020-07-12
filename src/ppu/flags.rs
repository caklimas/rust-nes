bitfield!{
    pub struct Status(u8);
    impl Debug;
    
    pub sprite_overflow, set_sprite_overflow: 5;
    pub sprite_zero_hit, set_sprite_zero_hit: 6;
    pub vertical_blank, set_vertical_blank: 7;
    pub get, _: 7, 0;
}

bitfield! {
    pub struct Mask(u8);
    impl Debug;

    pub greyscale, _: 0;
    pub render_background_left, _: 1;
    pub render_sprite_left, _: 2;
    pub render_background, _: 3;
    pub render_sprite, _: 4;
    pub emphasize_red, _: 5;
    pub emphasize_green, _: 6;
    pub emphasize_blue, _: 7;

    pub _, set: 7, 0;
}