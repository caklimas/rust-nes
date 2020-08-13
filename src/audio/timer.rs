use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Timer {
    pub counter: u16,
    pub period: u16
}

impl Timer {
    pub fn decrement(&mut self) {
        self.counter -= 1;
    }

    pub fn set_low(&mut self, data: u8) {
        self.period = (self.period & 0xFF00) | (data as u16);
    }

    pub fn set_high(&mut self, data: u8) {
        let period_high = ((data & 0b111) as u16) << 8;
        self.period = period_high | self.period & 0x00FF;
        self.counter = self.period;
    }

    pub fn reset(&mut self) {
        self.counter = self.period;
    }
}