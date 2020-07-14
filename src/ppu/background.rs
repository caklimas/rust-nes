#[derive(Debug, Default)]
pub struct Background {
    pub next_tile_id: u8,
    pub next_tile_attribute: u8,
    pub next_tile_lsb: u8,
    pub next_tile_msb: u8,
    pub shifter_pattern_low: u16,
    pub shifter_pattern_high: u16,
    pub shifter_attribute_low: u16,
    pub shifter_attribute_high: u16
}

impl Background {
    pub fn load_shifters(&mut self) {
        self.shifter_pattern_low = (self.shifter_pattern_low & 0xFF00) | (self.next_tile_lsb as u16);
        self.shifter_pattern_high = (self.shifter_pattern_high & 0xFF00) | (self.next_tile_msb as u16);

        // Attribute bits don't change per pixel, but for every tile(8 pixels)
        // We then inflate the bottom and top bit to 8 bits
        self.shifter_attribute_low = (self.shifter_attribute_low & 0xFF00) | (if (self.next_tile_attribute & 0b01) > 0 { 0xFF } else { 0x00 });
        self.shifter_attribute_high = (self.shifter_attribute_high & 0xFF00) | (if (self.next_tile_attribute & 0b10) > 0 { 0xFF } else { 0x00 });
    }

    pub fn update_shifters(&mut self) {
        self.shifter_pattern_low <<= 1;
        self.shifter_pattern_high <<= 1;
        self.shifter_attribute_low <<= 1;
        self.shifter_attribute_high <<= 1;
    }

    pub fn get_pixel(&mut self, fine_x_scroll: u8) -> (u8, u8) {
        let shift_register_bit = 0x8000 >> fine_x_scroll;

        let pixel_plane_0 = if (self.shifter_pattern_low & shift_register_bit) > 0 { 1 } else { 0 };
        let pixel_plane_1 = if (self.shifter_pattern_high & shift_register_bit) > 0 { 1 } else { 0 };
        let pixel = (pixel_plane_1 << 1) | pixel_plane_0;

        let palette_0 = if (self.shifter_attribute_low & shift_register_bit) > 0 { 1 } else { 0 };
        let palette_1 = if (self.shifter_attribute_high & shift_register_bit) > 0 { 1 } else { 0 };
        let palette = (palette_1 << 1) | palette_0;

        (palette, pixel)
    }
}