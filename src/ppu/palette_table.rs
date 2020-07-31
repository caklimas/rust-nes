pub struct PaletteTable {
    data: [u8; 32]
}

impl PaletteTable {
    pub fn new() -> Self {
        PaletteTable {
            data: [0; 32]
        }
    }

    pub fn read_data(&self, address: u16) -> u8 {
        let masked_address = PaletteTable::get_masked_address(address);
        self.data[masked_address as usize]
    }

    pub fn write_data(&mut self, address: u16, data: u8) {
        let masked_address = PaletteTable::get_masked_address(address);
        self.data[masked_address as usize] = data;
    }

    fn get_masked_address(address: u16) -> u16 {
        let address = address & 0x001F;
        let masked_address = match address & 0x001F {
            0x0010 => 0x0000,
            0x0014 => 0x0004,
            0x0018 => 0x0008,
            0x001C => 0x000C,
            _ => address
        };

        masked_address
    }
}