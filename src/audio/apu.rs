use crate::addresses;
use super::pulse;
use super::triangle;

const APU_CLOCK_RATE: u8 = 6;
const FRAME_COUNTER_STEPS: [usize; 5] = [3728, 7456, 11185, 14914, 18640];

#[derive(Debug)]
pub struct Apu2A03 {
    pub buffer: Vec<f32>,
    square_table: Vec<f32>,
    tnd_table: Vec<f32>,
    pulse_1: pulse::Pulse,
    pulse_2: pulse::Pulse,
    triangle: triangle::Triangle,
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
            tnd_table: (0..203).map(|x| 163.67/((24329.0 / x as f32) + 100.0)).collect(),
            pulse_1: pulse::Pulse::new(true),
            pulse_2: pulse::Pulse::new(false),
            triangle: Default::default(),
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
            
            if FRAME_COUNTER_STEPS.contains(&self.frame_clock_counter) {
                self.clock_frame_counter();
            }
            
            self.frame_clock_counter += 1;
            if self.is_max_step_counter() {
                self.frame_clock_counter = 0;
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
            addresses::APU_TRIANGLE_COUNTER_RELOAD => self.triangle.set_counter_reload(data),
            addresses::APU_TRIANGLE_TIMER_LOW => self.triangle.set_timer_low(data),
            addresses::APU_TRIANGLE_TIMER_HIGH => self.triangle.set_timer_high(data),
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

        if (data & 0b100) > 0 {
            self.triangle.enabled = true;
        } else {
            self.triangle.enabled = false;
            self.triangle.length_counter = 0;
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
        };

        if self.step_mode == 5 {
            self.clock_envelopes();
            self.clock_sweeps();
            self.clock_length_counters();
        }
    }

    fn mix_samples(&mut self) -> f32 {
        let pulse_1 = self.pulse_1.clock();
        let pulse_2 = self.pulse_2.clock();
        let triangle = self.triangle.clock();

        let pulse_index = pulse_1 + pulse_2;
        let pulse_out = self.square_table[pulse_index as usize];
        let tnd_index = 3 * triangle;
        let tnd_out = self.tnd_table[tnd_index as usize];

        pulse_out   
    }

    fn clock_4_step_frame_counter(&mut self) {
        match self.frame_clock_counter {
            3728 => {
                self.clock_envelopes();
                self.triangle.clock_linear_counter();
            },
            7456 => {
                self.clock_envelopes();
                self.clock_sweeps();
                self.clock_length_counters();
                self.triangle.clock_linear_counter();
            },
            11185 => {
                self.clock_envelopes();
                self.triangle.clock_linear_counter();
            },
            14914 => {
                self.clock_envelopes();
                self.clock_sweeps();
                self.clock_length_counters();
                self.triangle.clock_linear_counter();

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
                self.triangle.clock_linear_counter();
            },
            7456 => {
                self.clock_envelopes();
                self.clock_sweeps();
                self.clock_length_counters();
                self.triangle.clock_linear_counter();
            },
            11185 => {
                self.clock_envelopes();
                self.triangle.clock_linear_counter();
            },
            14914 => {
            },
            18640 => {
                self.clock_envelopes();
                self.clock_sweeps();
                self.clock_length_counters();
                self.triangle.clock_linear_counter();
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

    fn clock_length_counters(&mut self) {
        self.pulse_1.clock_length_counter();
        self.pulse_2.clock_length_counter();
        self.triangle.clock_length_counter();
    }

    fn is_max_step_counter(&mut self) -> bool {
        if self.step_mode == 4 {
            self.frame_clock_counter == 14915
        } else {
            self.frame_clock_counter == 18641
        }
    }
}