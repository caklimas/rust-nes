const RATE_TABLE: [u16; 16] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54
];

#[derive(Debug, Default)]
pub struct DeltaModulationChannel {
    pub remaining_bytes: u8,
    pub interrupt: bool,
    current_address: u16,
    irq_enabled: bool,
    loop_flag: bool,
    output_level: u8,
    sample_address: u16,
    sample_length: u16,
    sample_buffer: u8,
    rate: u16
}

impl DeltaModulationChannel {
    pub fn clock(&mut self) -> u8 {
        0
    }

    pub fn set_rate(&mut self, data: u8) {
        self.irq_enabled = (data & 0b10000000) > 0;
        self.loop_flag   = (data & 0b01000000) > 0;

        let rate_index = data & 0b00001111;
        self.rate = RATE_TABLE[rate_index as usize];
    }

    pub fn set_direct_load(&mut self, data: u8) {
        self.output_level = data & 0b01111111;
    }

    pub fn set_sample_address(&mut self, data: u8) {
        self.sample_address = 0xC000 + ((data as u16) * 64);
    }

    pub fn set_sample_length(&mut self, data: u8) {
        self.sample_length = ((data as u16) * 16) + 1;
    }
}