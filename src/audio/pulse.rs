use super::envelope;
use super::sweep;
use super::timer;

const DUTY_CYCLE_WAVEFORMS: [u8; 4] = [
    0b01000000, // 12.5%
    0b01100000, // 25%
    0b01111000, // 50%
    0b10011111, // 25% negated
];

#[derive(Debug)]
pub struct Pulse {
    pub envelope: envelope::Envelope,
    pub length_counter: u8,
    constant_volume: bool,
    duty_cycle: u8,
    duty_shifter: u8,
    enabled: bool,
    is_first: bool,
    sweep: sweep::Sweep,
    target_period: u16,
    timer: timer::Timer
}

impl Pulse {
    pub fn new(is_first: bool) -> Self {
        Pulse {
            envelope: Default::default(),
            length_counter: 0,
            constant_volume: false,
            duty_cycle: DUTY_CYCLE_WAVEFORMS[0],
            duty_shifter: 0,
            enabled: false,
            is_first,
            sweep: Default::default(),
            target_period: 0,
            timer: Default::default()
        }
    }

    pub fn clock(&mut self) -> u8 {
        if self.timer.counter == 0 {
            self.timer.reset();
            self.duty_shifter = (self.duty_shifter + 1) % 8;
        } else {
            self.timer.decrement();
        }

        let sample = ((self.duty_cycle >> (7 - self.duty_shifter)) & 0b01) as u16;
        if self.is_silenced(sample) {
            0
        } else if self.constant_volume {
            self.envelope.decay_counter_period
        } else {
            self.envelope.decay_counter
        }
    }

    pub fn clock_sweep(&mut self) {
        self.calculate_period();

        if self.sweep.divider_counter == 0 && self.sweep.enabled && !self.is_muting_channel() {
            self.timer.period = self.target_period;
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

    pub fn set_duty_cycle(&mut self, data: u8) {
        let duty = (data & 0b11000000) >> 6;
        self.duty_cycle = DUTY_CYCLE_WAVEFORMS[duty as usize];

        self.envelope.loop_flag = (data & 0b100000) > 0;
        self.constant_volume = (data & 0b10000) > 0;

        let volume = data & 0b1111;
        self.envelope.decay_counter_period = volume;
    }

    pub fn set_sweep(&mut self, data: u8) {
        self.sweep.enabled = (data & 0b10000000) > 0;
        self.sweep.period = ((data & 0b1110000) >> 4) as u16;
        self.sweep.negate = (data & 0b1000) > 0;
        self.sweep.shift_amount = data & 0b111;
        self.sweep.reload = true;
    }

    pub fn set_timer_low(&mut self, data: u8) {
        self.timer.set_low(data);
    }

    pub fn set_timer_high(&mut self, data: u8) {
        if self.enabled {
            let index = (data & 0b11111000) >> 3;
            self.length_counter = super::LENGTH_COUNTER_TABLE[index as usize];
        }

        self.timer.set_high(data);
        self.envelope.start = true;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.length_counter = 0;
        }
    }

    fn is_silenced(&self, sample: u16) -> bool {
        sample == 0 || self.length_counter == 0 || self.is_muting_channel()
    }

    fn is_muting_channel(&self) -> bool {
        self.timer.period < 8 || self.timer.period > 0x7FF
    }

    fn calculate_period(&mut self) {
        let period_change = self.timer.period >> self.sweep.shift_amount;

        if self.sweep.negate {
            self.target_period = self.timer.period - period_change;

            if self.is_first && self.target_period >= 1 {
                self.target_period -= 1;
            }
        } else {
            self.target_period = self.timer.period + period_change;
        }
    }
}