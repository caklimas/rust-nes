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