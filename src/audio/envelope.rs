use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Envelope {
    pub decay_counter: u8,
    pub loop_flag: bool,
    pub start: bool,
    pub volume: u8,
    divider: u8,
}

impl Envelope {
    pub fn clock(&mut self) {
        if !self.start {
            self.clock_divider();
        } else {
            self.start = false;
            self.decay_counter = 15;
            self.divider = self.volume;
        }
    }

    fn clock_divider(&mut self) {
        if self.divider == 0 {
            self.divider = self.volume;
            self.clock_delay_counter();
        } else {
            self.divider -= 1;
        }
    }

    fn clock_delay_counter(&mut self) {
        if self.decay_counter != 0 {
            self.decay_counter -= 1;
        } else if self.loop_flag {
            self.decay_counter = 15;
        }
    }
}