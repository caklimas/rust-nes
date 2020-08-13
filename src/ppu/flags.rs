use serde::{Serialize, Deserialize};
use crate::addresses::ppu::*;

bitfield!{
    #[derive(Serialize, Deserialize)]
    pub struct Status(u8);
    impl Debug;
    
    pub sprite_overflow, set_sprite_overflow: 5;
    pub sprite_zero_hit, set_sprite_zero_hit: 6;
    pub vertical_blank, set_vertical_blank: 7;
    pub get, _: 7, 0;
}

bitfield! {
    #[derive(Serialize, Deserialize)]
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

impl Mask {
    pub fn is_rendering_enabled(&mut self) -> bool {
        self.render_background() || self.render_sprite()
    }
}

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct Control(u8);
    impl Debug;

    pub name_table_address, _: 1, 0;
    pub vram_address, _: 2;
    pub sprite_table_address, _: 3;
    pub background_table_address, _: 4;
    pub sprite_size, _: 5;
    pub ppu_master_slave_select, _: 6;
    pub generate_nmi, _: 7;

    pub _, set: 7, 0;
}

impl Control {
    pub fn get_increment_amount(&mut self) -> u16 {
        if self.vram_address() { 32 } else { 1 }
    }

    pub fn get_sprite_size(&mut self) -> u8 {
        if self.sprite_size() { 16 } else { 8 }
    }
}

bitfield! {
    #[derive(Serialize, Deserialize, Copy, Clone)]
    pub struct ScrollAddress(u16);
    impl Debug;

    pub u8, coarse_x, set_coarse_x: 4, 0;
    pub u8, coarse_y, set_coarse_y: 9, 5;
    pub u8, name_table_x, set_name_table_x: 10;
    pub u8, name_table_y, set_name_table_y: 11;
    pub u8, name_table, set_name_table: 11, 10;
    pub u8, fine_y, set_fine_y: 14, 12;
    pub u8, low_byte, set_low_byte: 7, 0;
    pub u8, high_byte, set_high_byte: 13, 8;
    pub u16, get, set: 14, 0;
}

impl ScrollAddress {
    pub fn increment(&mut self, amount: u16) {
        self.0 = self.0.wrapping_add(amount);
    }

    pub fn increment_x(&mut self) {
        if self.coarse_x() == 31 {
            self.set_coarse_x(0);
            self.set_name_table_x(!self.name_table_x()); // Switch horizontal table
        } else {
            self.set_coarse_x(self.coarse_x() + 1);
        }
    }

    pub fn increment_y(&mut self) {
        if self.fine_y() < 7 {
            self.set_fine_y(self.fine_y() + 1);
        } else {
            self.set_fine_y(0);
            let mut y = self.coarse_y();
            if y == 29 { // 29 is the last row of tiles in the name table
                y = 0;
                self.set_name_table_y(!self.name_table_y()); // Switch vertical table
            } else if y == 31 { // Coarse Y can be set out of bounds and will wrap to 0
                y = 0;
            } else {
                y += 1;
            }

            self.set_coarse_y(y);
        }
    }

    pub fn transfer_x_address(&mut self, source: ScrollAddress) {
        self.set_name_table_x(source.name_table_x());
        self.set_coarse_x(source.coarse_x());
    }

    pub fn transfer_y_address(&mut self, source: ScrollAddress) {
        self.set_fine_y(source.fine_y());
        self.set_name_table_y(source.name_table_y());
        self.set_coarse_y(source.coarse_y());
    }

    pub fn name_table_address(&mut self) -> u16 {
        NAME_TABLE_ADDRESS_LOWER | (self.get() & 0x0FFF)
    }

    pub fn attribute_table_address(&mut self) -> u16 {
        ATTRIBUTE_TABLE_ADDRESS_LOWER |
        ((self.name_table() as u16) << 10) |
        ((self.coarse_y() >> 2) << 3) as u16 |
        (self.coarse_x() >> 2) as u16
    }
}