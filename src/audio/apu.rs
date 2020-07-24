use crate::addresses;
use super::pulse;

const APU_CLOCK_RATE: u8 = 6;
const FRAME_COUNTER_STEPS: [usize; 5] = [3728, 7456, 11185, 14914, 18640];

#[derive(Debug)]
pub struct Apu2A03 {
    pub buffer: Vec<f32>,
    square_table: Vec<f32>,
    pulse_1: pulse::Pulse,
    pulse_2: pulse::Pulse,
    clock_counter: u32,
    frame_clock_counter: usize, // Maintains musical timing of the apu
    step_mode: u8,
    interrupt_inhibit: bool,
    trigger_interrupt: bool
}

impl Apu2A03 {
    pub fn initialize() -> Self {
        Apu2A03 {
            buffer: Vec::<f32>::new(),
            square_table: (0..31).map(|x| 95.52/((8128.0 / x as f32) + 100.0)).collect(),
            pulse_1: pulse::Pulse::new(true),
            pulse_2: pulse::Pulse::new(false),
            clock_counter: 0,
            frame_clock_counter: 0,
            step_mode: 0,
            interrupt_inhibit: false,
            trigger_interrupt: false
        }
    }

    pub fn reset(&mut self) {

    }

    pub fn clock(&mut self) {
        if self.clock_counter % (APU_CLOCK_RATE as u32) == 0 {
            self.frame_clock_counter += 1;

            if FRAME_COUNTER_STEPS.contains(&self.frame_clock_counter) {
                self.clock_frame_counter();
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
            addresses::APU_PULSE_1_DUTY => self.pulse_1.set_duty_cycle(data),
            addresses::APU_PULSE_1_SWEEP => self.pulse_1.set_sweep(data),
            addresses::APU_PULSE_1_TIMER_LOW => self.pulse_1.set_reload_low(data),
            addresses::APU_PULSE_1_TIMER_HIGH => self.pulse_1.set_reload_high(data),
            addresses::APU_PULSE_2_DUTY => self.pulse_2.set_duty_cycle(data),
            addresses::APU_PULSE_2_SWEEP => self.pulse_2.set_sweep(data),
            addresses::APU_PULSE_2_TIMER_LOW => self.pulse_2.set_reload_low(data),
            addresses::APU_PULSE_2_TIMER_HIGH => self.pulse_2.set_reload_high(data),
            addresses::APU_NOISE_1 => {

            },
            addresses::APU_NOISE_2 => {
                
            },
            addresses::APU_STATUS => self.write_status(data),
            addresses::APU_FRAME_COUNTER => self.write_frame_counter(data),
            _ => ()
        }
    }

    fn clock_frame_counter(&mut self) {
        match self.step_mode {
            4 => {
                self.clock_4_step_frame_counter();
            },
            5 => {
                self.clock_5_step_frame_counter();
            },
            _ => ()
        }
    }

    fn write_status(&mut self, data: u8) {
        if (data & 0b01) > 0 {
            self.pulse_1.enabled = true;
        } else {
            self.pulse_1.enabled = false;
            self.pulse_1.length_counter = 0;
        }

        if (data & 0b10) > 0 {
            self.pulse_2.enabled = true;
        } else {
            self.pulse_2.enabled = false;
            self.pulse_2.length_counter = 0;
        }
    }

    fn write_frame_counter(&mut self, data: u8) {
        self.step_mode = match data >> 7 {
            0 => {
                4
            },
            1 => {
                5
            },
            _ => panic!("Invalid step mode")
        };

        self.interrupt_inhibit = match (data >> 6) & 0b1 {
            0 => {
                false
            },
            1 => {
                true
            },
            _ => panic!("Invalid interrupt request")
        }
    }

    fn mix_samples(&mut self) -> f32 {
        let square_1 = self.pulse_1.clock();
        let square_2 = self.pulse_2.clock();

        self.square_table[(square_1 + square_2) as usize]
    }

    fn clock_4_step_frame_counter(&mut self) {
        match self.frame_clock_counter {
            3728 => {
                self.clock_envelopes();
            },
            7456 => {
                self.clock_envelopes();
                self.clock_sweeps();
            },
            11185 => {
                self.clock_envelopes();
            },
            14914 => {
                self.clock_envelopes();
                self.clock_sweeps();

                if !self.interrupt_inhibit {
                    self.trigger_interrupt = true;
                } 
            },
            _ => ()
        }
    }

    fn clock_5_step_frame_counter(&mut self) {
        match self.frame_clock_counter {
            3728 => {
                self.clock_envelopes();
            },
            7456 => {
                self.clock_envelopes();
                self.clock_sweeps();
            },
            11185 => {
                self.clock_envelopes();
            },
            14914 => {
            },
            18640 => {
                self.clock_envelopes();
                self.clock_sweeps();
            },
            _ => ()
        }
    }

    fn clock_envelopes(&mut self) {
        self.pulse_1.envelope.clock();
        self.pulse_2.envelope.clock();
    }

    fn clock_sweeps(&mut self) {
        self.pulse_1.clock_sweep();
        self.pulse_2.clock_sweep();
    }
}