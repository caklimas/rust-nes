use super::envelope;
use super::sweep;

const DUTY_CYCLE_WAVEFORMS: [u8; 4] = [
    0b01000000, // 12.5%
    0b01100000, // 25%
    0b01111000, // 50%
    0b10011111, // 25% negated
];

#[derive(Debug)]
pub struct Pulse {
    pub is_first: bool,
    pub enabled: bool,
    pub envelope: envelope::Envelope,
    pub constant_volume: bool,
    pub length_counter: u8,
    pub sample: u16,
    pub sweep: sweep::Sweep,
    duty_cycle: u8,
    duty_shifter: u8,
    timer: u16,
    timer_period: u16,
    target_period: u16
}

impl Pulse {
    pub fn new(is_first: bool) -> Self {
        Pulse {
            is_first,
            enabled: false,
            envelope: Default::default(),
            constant_volume: false,
            length_counter: 0,
            sample: 0,
            sweep: Default::default(),
            duty_cycle: DUTY_CYCLE_WAVEFORMS[0],
            duty_shifter: 0,
            timer: 0,
            timer_period: 0,
            target_period: 0
        }
    }

    pub fn clock(&mut self) -> u16 {
        if self.timer == 0 {
            self.timer = self.timer_period;
            self.duty_shifter = (self.duty_shifter + 1) % 8;
        } else {
            self.timer -= 1;
        }

        let sample = ((self.duty_cycle >> (7 - self.duty_shifter)) & 0b01) as u16;
        return if self.is_silenced(sample) {
            0
        } else if self.constant_volume {
            self.envelope.decay_counter_period
        } else {
            self.envelope.decay_counter
        }
    }

    pub fn set_duty_cycle(&mut self, data: u8) {
        let duty = (data & 0b11000000) >> 6;
        self.duty_cycle = DUTY_CYCLE_WAVEFORMS[duty as usize];

        self.envelope.loop_flag = (data & 0b100000) > 0;
        self.constant_volume = (data & 0b10000) > 0;

        let volume = data & 0b1111;
        self.envelope.decay_counter_period = volume as u16;
    }

    pub fn clock_sweep(&mut self) {
        self.calculate_period();

        if self.sweep.divider_counter == 0 && self.sweep.enabled && !self.is_muting_channel() {
            self.timer_period = self.target_period;
        }

        if self.sweep.divider_counter == 0 || self.sweep.reload {
            self.sweep.divider_counter = self.sweep.period;
            self.sweep.reload = false;
        } else {
            self.sweep.divider_counter -= 1;
        }
    }

    pub fn clock_length_counter(&mut self) {
        if self.length_counter > 0 && !self.envelope.loop_flag {
            self.length_counter -= 1;
        }
    }

    pub fn set_sweep(&mut self, data: u8) {
        self.sweep.enabled = (data & 0b10000000) > 0;
        self.sweep.period = ((data & 0b1110000) >> 4) as u16;
        self.sweep.negate = (data & 0b1000) > 0;
        self.sweep.shift_amount = data & 0b111;
        self.sweep.reload = true;
    }

    pub fn set_reload_low(&mut self, data: u8) {
        self.timer_period = (self.timer_period & 0xFF00) | (data as u16);
    }

    pub fn set_reload_high(&mut self, data: u8) {
        if self.enabled {
            let index = (data & 0b11111000) >> 3;
            self.length_counter = super::LENGTH_COUNTER_TABLE[index as usize];
        }

        let reload_high = ((data & 0b111) as u16) << 8;
        self.timer_period = reload_high | self.timer_period & 0x00FF;
        self.timer = self.timer_period;
        self.envelope.start = true;
    }

    fn is_silenced(&mut self, sample: u16) -> bool {
        sample == 0 || self.length_counter == 0 || self.is_muting_channel()
    }

    fn is_muting_channel(&mut self) -> bool {
        self.timer_period < 8 || self.timer_period > 0x7FF
    }

    fn calculate_period(&mut self) {
        let period_change = self.timer_period >> self.sweep.shift_amount;

        if self.sweep.negate {
            self.target_period = self.timer_period - period_change;

            if self.is_first && self.target_period >= 1 {
                self.target_period -= 1;
            }
        } else {
            self.target_period = self.timer_period + period_change;
        }
    }
}