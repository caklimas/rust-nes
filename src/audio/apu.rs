use crate::addresses;

#[derive(Debug, Default)]
pub struct Apu {
    pulse_1_enable: bool,
    pulse_1_sample: f32
}

impl Apu {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn reset(&mut self) {

    }

    pub fn clock(&mut self) {

    }

    pub fn read(&mut self, address: u16) -> u8 {
        let data = 0;

        data
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            addresses::APU_PULSE_1_TIMER => {

            },
            addresses::APU_PULSE_1_LENGTH_COUNTER => {

            },
            addresses::APU_PULSE_1_ENVELOPE => {

            },
            addresses::APU_PULSE_1_SWEEP => {

            },
            addresses::APU_PULSE_2_TIMER => {

            },
            addresses::APU_PULSE_2_LENGTH_COUNTER => {

            },
            addresses::APU_PULSE_2_ENVELOPE => {

            },
            addresses::APU_PULSE_2_SWEEP => {

            },
            addresses::APU_NOISE_1 => {

            },
            addresses::APU_NOISE_2 => {
                
            }
            _ => ()
        }
    }

    pub fn get_output_sample(&mut self) -> f32 {
        self.pulse_1_sample
    }
}