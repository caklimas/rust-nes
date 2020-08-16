use crate::addresses::ppu::*;

const CONTROL: u16 = 0x0000; // Configure ppu to render in different ways
const MASK: u16 = 0x0001; // Decides what sprites or backgrounds are being drawn and what happens at the edges of the screen
const STATUS: u16 = 0x0002;
const OAM_ADDRESS: u16 = 0x0003;
const OAM_DATA: u16 = 0x0004;
const SCROLL: u16 = 0x0005; // Used for worlds larger than the current screen
const PPU_ADDRESS: u16 = 0x0006; // The ppu address to send data to
const PPU_DATA: u16 = 0x0007; // The data to send to the ppu address

impl super::Ppu2C02 {
    /// Read from the Main Bus
    pub fn read(&mut self, address: u16) -> u8 {
        let masked_address = address & PPU_ADDRESS_RANGE;
        match masked_address {
            CONTROL | MASK | OAM_ADDRESS | SCROLL | PPU_ADDRESS => 0, // Can't be read
            STATUS => self.read_status(),
            OAM_DATA => self.read_oam_data(),
            PPU_DATA => self.read_ppu_data(),
            _ => 0
        }
    }

    /// Write to the Main Bus
    pub fn write(&mut self, address: u16, data: u8) {
        let masked_address = address & PPU_ADDRESS_RANGE; 
        match masked_address {
            CONTROL => self.write_control(data),
            MASK => self.write_mask(data),
            STATUS => (),
            OAM_ADDRESS => self.write_oam_address(data),
            OAM_DATA => self.write_oam_data(data),
            SCROLL => self.write_ppu_scroll(data),
            PPU_ADDRESS => self.write_ppu_address(data),
            PPU_DATA => self.write_ppu_data(data),
            _ => ()
        };
    }

    fn read_status(&mut self) -> u8 {
        let data = self.status.get() | (self.ppu_data_buffer & 0x1F);
        self.status.set_vertical_blank(false);
        self.address_latch = false;
        data
    }

    fn read_oam_data(&mut self) -> u8 {
        self.oam.memory[self.oam.address as usize]
    }

    fn read_ppu_data(&mut self) -> u8 {
        let mut data = self.ppu_data_buffer;
        self.ppu_data_buffer = self.ppu_read(self.current_vram_address.get());

        if self.current_vram_address.get() >= PALETTE_ADDRESS_LOWER {
            data = self.ppu_data_buffer;
        }

        if self.mask.is_rendering_enabled() && (self.scanline < 240 || self.scanline == 261) {
            self.current_vram_address.increment_x();
            self.current_vram_address.increment_y();

        } else {
            self.current_vram_address.increment(self.control.get_increment_amount());
        }

        data
    }

    fn write_control(&mut self, data: u8) {
        self.control.set(data);

        // t: ...BA.......... = d: ......BA
        let ba = data & 0b11;
        self.temp_vram_address.set_name_table(ba);
    }

    fn write_mask(&mut self, data: u8) {
        self.mask.set(data);
    }

    fn write_oam_address(&mut self, data: u8) {
        self.oam.address = data;
    }

    fn write_oam_data(&mut self, data: u8) {
        self.oam.memory[self.oam.address as usize] = data;
        self.oam.address = self.oam.address.wrapping_add(1);
    }

    fn write_ppu_scroll(&mut self, data: u8) {
        if !self.address_latch {
            // t: ....... ...HGFED = d: HGFED...
            // x:              CBA = d: .....CBA
            // w:                  = 1
            let hgfed = (data & 0b11111000) >> 3;
            self.temp_vram_address.set_coarse_x(hgfed);
            self.fine_x_scroll = data & 0b111;
            self.address_latch = true;
        } else {
            // t: CBA..HGFED..... = d: HGFEDCBA
            // w:                  = 0
            let cba = data & 0b111;
            let hgfed = (data & 0b11111000) >> 3;
            self.temp_vram_address.set_coarse_y(hgfed);
            self.temp_vram_address.set_fine_y(cba);

            self.address_latch = false;
        }
    }

    fn write_ppu_address(&mut self, data: u8) {
        if !self.address_latch {
            // t: .FEDCBA........ = d: ..FEDCBA
            // t: X.............. = 0
            // w:                  = 1
            let fedbca = data & 0b00111111;
            self.temp_vram_address.set_high_byte(fedbca);
            self.address_latch = true;
        } else {
            // t: ....... HGFEDCBA = d: HGFEDCBA
            // v                   = t
            // w:                  = 0
            self.temp_vram_address.set_low_byte(data);

            self.current_vram_address.set(self.temp_vram_address.get());
            self.address_latch = false;
        }
    }

    fn write_ppu_data(&mut self, data: u8) {
        self.ppu_write(self.current_vram_address.get(), data);
        if self.current_vram_address.get() == 0x3f12 {
            // println!("PPU_DATA writing data {} to address {}", data, self.current_vram_address.get());
        }

        if self.mask.is_rendering_enabled() && (self.scanline < 240 || self.scanline == 261) {
            self.current_vram_address.increment_x();
            self.current_vram_address.increment_y();

        } else {
            self.current_vram_address.increment(self.control.get_increment_amount());
        }
    }
}