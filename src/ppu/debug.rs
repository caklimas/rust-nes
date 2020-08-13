use std::fmt::{Debug, Formatter, Result};

impl Debug for super::Ppu2C02 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Ppu2C02")
         .field("nmi", &self.nmi)
         .field("frame", &self.frame)
         .field("oam", &self.oam)
         .field("address_latch", &self.address_latch)
         .field("background", &self.background)
         .field("control", &self.control)
         .field("current_vram_address", &self.current_vram_address)
         .field("cycle", &self.cycle)
         .field("fine_x_scroll", &self.fine_x_scroll)
         .field("mask", &self.mask)
         .field("name_table", &self.name_table)
         .field("pallete_table", &self.pallete_table)
         .field("pattern_table", &self.pattern_table)
         .field("ppu_data_buffer", &self.ppu_data_buffer)
         .field("scanline", &self.scanline)
         .field("sprite", &self.sprite)
         .field("status", &self.status)
         .field("temp_vram_address", &self.temp_vram_address)
         .finish()
    }
}