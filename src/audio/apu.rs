use crate::addresses;
use super::pulse;
use super::sequencer;

const APU_CLOCK_RATE: u8 = 6;

#[derive(Debug)]
pub struct Apu2A03 {
    pub buffer: Vec<f32>,
    square_table: Vec<f32>,
    pulse1: pulse::Pulse,
    clock_counter: u32,
    frame_clock_counter: u32 // Maintains musical timing of the apu
}

impl Apu2A03 {
    pub fn initialize() -> Self {
        Apu2A03 {
            buffer: Vec::<f32>::new(),
            square_table: (0..31).map(|x| 95.52/((8128.0 / x as f32) + 100.0)).collect(),
            pulse1: pulse::Pulse::new(),
            clock_counter: 0,
            frame_clock_counter: 0
        }
    }

    pub fn reset(&mut self) {

    }

    pub fn clock(&mut self) {
        if self.clock_counter % (APU_CLOCK_RATE as u32) == 0 {
            self.frame_clock_counter += 1;

            let (quarter_frame_clock, half_frame_clock) = self.get_4_step_sequence_flags();

            if quarter_frame_clock {
                self.adjust_volume_envelope();
            }

            if half_frame_clock {
                self.adjust_note_length();
                self.adjust_frequency_sweepers();
            }

            let sample = self.mix_samples();
            self.buffer.push(sample);
        }

        self.clock_counter += 1;
    }

    pub fn read(&mut self, address: u16) -> u8 {
        let data = 0;

        data
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            addresses::APU_PULSE_1_DUTY => {
                self.pulse1.set_duty_cycle((data & 0b11000000) >> 6);
            },
            addresses::APU_PULSE_1_SWEEP => {
            },
            addresses::APU_PULSE_1_TIMER_LOW => {
                self.pulse1.set_reload_low(data);
            },
            addresses::APU_PULSE_1_TIMER_HIGH => {  
                self.pulse1.set_reload_high(data);
            },
            addresses::APU_PULSE_2_DUTY => {

            },
            addresses::APU_PULSE_2_SWEEP => {

            },
            addresses::APU_PULSE_2_TIMER_LOW => {

            },
            addresses::APU_PULSE_2_TIMER_HIGH => {

            },
            addresses::APU_NOISE_1 => {

            },
            addresses::APU_NOISE_2 => {
                
            },
            addresses::APU_STATUS => {
                if (data & 0x01) > 0 {
                    self.pulse1.enabled = true;
                } else {
                    self.pulse1.enabled = false;
                    self.pulse1.length_counter = 0;
                }
            },
            _ => ()
        }
    }

    fn get_4_step_sequence_flags(&mut self) -> (bool, bool) {
        let mut quarter_frame_clock = false;
        let mut half_frame_clock = false;

        match self.frame_clock_counter {
            3729 => {
                quarter_frame_clock = true;
            },
            7457 => {
                quarter_frame_clock = true;
                half_frame_clock = true;
            },
            11186 => {
                quarter_frame_clock = true;
            },
            14916 => {
                quarter_frame_clock = true;
                half_frame_clock = true;
                self.frame_clock_counter = 0;
            },
            _ => ()
        }

        (quarter_frame_clock, half_frame_clock)
    }

    fn adjust_volume_envelope(&mut self) {

    }

    fn adjust_note_length(&mut self) {

    }

    fn adjust_frequency_sweepers(&mut self) {

    }

    fn mix_samples(&mut self) -> f32 {
        let square1 = self.pulse1.clock();

        self.square_table[square1 as usize]
    }
}