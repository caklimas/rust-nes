use serde::{Serialize, Deserialize};
use super::envelope;

const PERIOD_TABLE: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068
];

#[derive(Serialize, Deserialize, Debug)]
pub struct Noise {
    pub envelope: envelope::Envelope,
    pub length_counter: u8,
    constant_volume: bool,
    enabled: bool,
    feedback_shift: u16,
    mode: bool,
    period: u16,
    timer: u16
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            envelope: Default::default(),
            length_counter: 0,
            constant_volume: false,
            enabled: false,
            feedback_shift: 1, // On power-up, the shift register is loaded with the value 1.
            mode: false,
            period: 0,
            timer: 0
        }
    }

    pub fn clock(&mut self) -> u8 {
        if self.timer == 0 {
            self.clock_shift_register();
        } else {
            self.timer -= 1;
        }
        
        if self.is_silenced() {
            0
        } else if self.constant_volume {
            self.envelope.volume
        } else {
            self.envelope.decay_counter
        }
    }

    pub fn clock_length_counter(&mut self) {
        if self.length_counter > 0 && !self.envelope.loop_flag {
            self.length_counter -= 1;
        }
    }

    /// 0x400C
    pub fn set_volume(&mut self, data: u8) {
        self.envelope.loop_flag = (data & 0b100000) > 0;
        self.constant_volume = (data & 0b10000) > 0;

        let volume = data & 0b1111;
        self.envelope.volume = volume;
    }

    /// 0x400E
    pub fn set_period(&mut self, data: u8) {
        self.mode = data & 0b10000000 > 0;
        let period_index = (data & 0b1111) as usize;
        self.period = PERIOD_TABLE[period_index];
    }

    pub fn set_length_counter(&mut self, data: u8) {
        self.length_counter = (data & 0b11111000) >> 3;
        self.envelope.start = true;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.length_counter = 0;
        }
    }

    fn clock_shift_register(&mut self) {
        let first_bit = self.feedback_shift & 0b1;
        let bit_shift = if self.mode {
            6
        } else {
            1
        };

        let other_bit = (self.feedback_shift & (1 << bit_shift)) >> bit_shift;
        let calculated_feedback = first_bit ^ other_bit;
        self.feedback_shift >>= 1;

        // Bit 14, the leftmost bit, is set to the calculated feedback.
        self.feedback_shift |= calculated_feedback << 14; 
    }

    fn is_silenced(&mut self) -> bool {
         (self.feedback_shift & 0b1) == 1 || self.length_counter == 0
    }
}