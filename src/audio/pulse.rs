const DUTY_CYCLE_SEQUENCES: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

#[derive(Debug)]
pub struct Pulse {
    pub enabled: bool,
    pub length_counter: u8,
    pub sample: u16,
    duty_cycle: [u8; 8],
    duty_counter: usize,
    timer: u16,
    reload: u16
}

impl Pulse {
    pub fn new() -> Self {
        Pulse {
            enabled: false,
            length_counter: 0,
            sample: 0,
            duty_cycle: DUTY_CYCLE_SEQUENCES[0],
            duty_counter: 0,
            timer: 0,
            reload: 0
        }
    }

    pub fn clock(&mut self) -> u16 {
        if self.timer == 0 {
            self.timer = self.reload;
            self.duty_counter = (self.duty_counter + 1) % 8;
        } else {
            self.timer -= 1;
        }

        let sample = self.duty_cycle[self.duty_counter] as u16;
        sample
    }

    pub fn set_duty_cycle(&mut self, duty: u8) {
        self.duty_cycle = DUTY_CYCLE_SEQUENCES[duty as usize];
    }

    pub fn set_reload_low(&mut self, data: u8) {
        self.reload = (self.reload & 0xFF00) | (data as u16);
    }

    pub fn set_reload_high(&mut self, data: u8) {
        let reload_high = ((data & 0b111) as u16) << 8;
        self.reload = reload_high | self.reload & 0x00FF;
        self.timer = self.reload;
    }
}