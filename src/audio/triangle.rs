const SEQUENCER_LENGTH: usize = 32;
const SEQUENCER: [u8; SEQUENCER_LENGTH] = [
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
];

#[derive(Debug, Default)]
pub struct Triangle {
    pub counter_reload: u8,
    pub enabled: bool,
    pub length_counter: u8,
    pub length_counter_halt: bool,
    pub linear_counter: u8,
    pub linear_counter_reload: bool,
    pub sample: u16,
    pub reload: u16,
    sequencer_counter: usize,
    timer: u16
}

impl Triangle {
    pub fn clock(&mut self) -> u8 {
        self.get_sample();
        self.get_sample()
    }
    
    pub fn clock_length_counter(&mut self) {
        if self.length_counter > 0 && !self.length_counter_halt {
            self.length_counter -= 1;
        }
    }

    pub fn clock_linear_counter(&mut self) {
        if self.linear_counter_reload {
            self.linear_counter = self.counter_reload;
        } else if self.linear_counter != 0 {
            self.linear_counter -= 1;
        }

        if !self.length_counter_halt {
            self.linear_counter_reload = false;
        }
    }

    pub fn set_counter_reload(&mut self, data: u8) {
        self.length_counter_halt = (data & 0b10000000) > 0;
        self.counter_reload = data & 0b01111111;
    }

    pub fn set_timer_low(&mut self, data: u8) {
        let timer_high = self.timer & 0xFF00;
        self.timer = timer_high | (data as u16);
    }

    pub fn set_timer_high(&mut self, data: u8) {
        let timer_low = self.timer & 0xFF;
        let timer_high = ((data & 0b111) as u16) << 8;
        self.timer = timer_high | timer_low;
        self.linear_counter_reload = true;

        if self.enabled {
            let index = ((data & 0b11111000) >> 3) as usize;
            self.length_counter = super::LENGTH_COUNTER_TABLE[index];
        }
    }

    fn get_sample(&mut self) -> u8 {
        if self.timer == 0 {
            self.timer = self.reload;
            if self.length_counter != 0 && self.linear_counter != 0 {
                self.sequencer_counter = (self.sequencer_counter + 1) % SEQUENCER_LENGTH;
            }
        } else {
            self.timer -= 1;
        }

        SEQUENCER[self.sequencer_counter]
    }
}