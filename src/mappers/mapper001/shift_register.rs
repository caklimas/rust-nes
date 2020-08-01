#[derive(Debug, Default)]
pub struct ShiftRegister {
    pub bit_shift: u8,
    pub load_register: u8
}

impl ShiftRegister {
    pub fn push_data(&mut self, data: u8) {
        self.load_register |= (data & 0b1) << self.bit_shift;
        self.bit_shift += 1;
    }

    pub fn reset(&mut self) {
        self.bit_shift = 0;
        self.load_register = 0;
    }

    pub fn reset_loading(&mut self) {
        self.load_register = 0;
        self.bit_shift = 0;
    }
}