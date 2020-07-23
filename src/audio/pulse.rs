use super::envelope;

const DUTY_CYCLE_WAVEFORMS: [u8; 4] = [
    0b01000000, // 12.5%
    0b01100000, // 25%
    0b01111000, // 50%
    0b10011111, // 25% negated
];

#[derive(Debug)]
pub struct Pulse {
    pub enabled: bool,
    pub envelope: envelope::Envelope,
    pub length_counter: u8,
    pub sample: u16,
    duty_cycle: u8,
    duty_shifter: u8,
    timer: u16,
    reload: u16
}

impl Pulse {
    pub fn new() -> Self {
        Pulse {
            enabled: false,
            envelope: Default::default(),
            length_counter: 0,
            sample: 0,
            duty_cycle: DUTY_CYCLE_WAVEFORMS[0],
            duty_shifter: 0,
            timer: 0,
            reload: 0
        }
    }

    pub fn clock(&mut self) -> u16 {
        if self.timer == 0 {
            self.timer = self.reload;
            self.duty_shifter = (self.duty_shifter + 1) % 8;
        } else {
            self.timer -= 1;
        }

        let sample = ((self.duty_cycle >> (7 - self.duty_shifter)) & 0b01) as u16;
        return if self.is_silenced(sample) {
            0
        } else {
            sample
        }
    }

    pub fn set_duty_cycle(&mut self, data: u8) {
        let duty = (data & 0b11000000) >> 6;
        self.duty_cycle = DUTY_CYCLE_WAVEFORMS[duty as usize];
    }

    pub fn set_sweep(&mut self, data: u8) {

    }

    pub fn set_reload_low(&mut self, data: u8) {
        self.reload = (self.reload & 0xFF00) | (data as u16);
    }

    pub fn set_reload_high(&mut self, data: u8) {
        let reload_high = ((data & 0b111) as u16) << 8;
        self.reload = reload_high | self.reload & 0x00FF;
        self.timer = self.reload;
    }

    fn is_silenced(&mut self, sample: u16) -> bool {
        return sample == 0 || self.reload < 8;
    }
}