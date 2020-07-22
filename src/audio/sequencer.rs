#[derive(Debug, Default)]
pub struct Sequencer {
    pub sequence: u32,
    pub timer: u16,
    pub reload: u16,
    pub output: u8
}

impl Sequencer {
    pub fn clock(&mut self, enable: bool, manipulate_sequence: fn(sequence: &mut u32)) {
        if !enable {
            return;
        }

        self.timer = self.timer.wrapping_sub(1);
        if self.timer == 0xFFFF {
            self.timer = self.reload + 1;
            manipulate_sequence(&mut self.sequence);
            self.output = (self.sequence & 0x00000001) as u8;
        }
    }

    pub fn set_reload_low(&mut self, data: u8) {
        self.reload = (self.reload & 0xFF00) | (data as u16);
    }

    pub fn set_reload_high(&mut self, data: u8) {
        let reload_high = ((data & 0b111) as u16) << 8;
        self.reload = reload_high | self.reload & 0x00FF;
    }
}