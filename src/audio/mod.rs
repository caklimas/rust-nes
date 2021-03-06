pub mod device;
pub mod dmc;
pub mod envelope;
pub mod filter;
pub mod noise;
pub mod pulse;
pub mod sweep;
pub mod timer;
pub mod triangle;

use serde::{Serialize, Deserialize};
use crate::addresses::apu::*;

const LENGTH_COUNTER_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12,
    16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30
];

const SAMPLE_RATE: i32 = 44_100;
const FRAME_COUNTER_STEPS: [usize; 5] = [3728, 7456, 11185, 14914, 18640];

#[derive(Serialize, Deserialize, Debug)]
pub struct Apu2A03 {
    pub buffer: Vec<f32>,
    pub trigger_interrupt: bool,
    clock_counter: u32,
    dmc: dmc::DeltaModulationChannel,
    frame_clock_counter: usize, // Maintains musical timing of the apu
    frame_interrupt: bool,
    interrupt_inhibit: bool,
    noise: noise::Noise,
    pulse_1: pulse::Pulse,
    pulse_2: pulse::Pulse,
    square_table: Vec<f32>,
    step_mode: u8,
    tnd_table: Vec<f32>,
    triangle: triangle::Triangle
}

impl Apu2A03 {
    pub fn initialize() -> Self {
        Apu2A03 {
            buffer: Vec::<f32>::new(),
            trigger_interrupt: false,
            clock_counter: 0,
            dmc: Default::default(),
            frame_clock_counter: 0,
            frame_interrupt: false,
            interrupt_inhibit: false,
            noise: noise::Noise::new(),
            pulse_1: pulse::Pulse::new(true),
            pulse_2: pulse::Pulse::new(false),
            square_table: (0..31).map(|x| 95.52/((8128.0 / x as f32) + 100.0)).collect(),
            step_mode: 0,
            triangle: Default::default(),
            tnd_table: (0..203).map(|x| 163.67/((24329.0 / x as f32) + 100.0)).collect()
        }
    }

    pub fn reset(&mut self) {

    }

    pub fn clock(&mut self) {    
        let sample = self.mix_samples();

        if FRAME_COUNTER_STEPS.contains(&self.frame_clock_counter) {
            self.clock_frame_counter();
        }
        
        self.frame_clock_counter += 1;
        if self.is_max_step_counter() {
            self.frame_clock_counter = 0;
        }
        self.buffer.push(sample);
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            APU_STATUS => self.read_status(),
            _ => 0
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            APU_PULSE_1_DUTY => self.pulse_1.set_duty_cycle(data),
            APU_PULSE_1_SWEEP => self.pulse_1.set_sweep(data),
            APU_PULSE_1_TIMER_LOW => self.pulse_1.set_timer_low(data),
            APU_PULSE_1_TIMER_HIGH => self.pulse_1.set_timer_high(data),
            APU_PULSE_2_DUTY => self.pulse_2.set_duty_cycle(data),
            APU_PULSE_2_SWEEP => self.pulse_2.set_sweep(data),
            APU_PULSE_2_TIMER_LOW => self.pulse_2.set_timer_low(data),
            APU_PULSE_2_TIMER_HIGH => self.pulse_2.set_timer_high(data),
            APU_TRIANGLE_COUNTER_RELOAD => self.triangle.set_counter_reload(data),
            APU_TRIANGLE_TIMER_LOW => self.triangle.set_timer_low(data),
            APU_TRIANGLE_TIMER_HIGH => self.triangle.set_timer_high(data),
            APU_NOISE_VOLUME => self.noise.set_volume(data),
            APU_NOISE_PERIOD => self.noise.set_period(data),
            APU_NOISE_COUNTER_LOAD => self.noise.set_length_counter(data),
            APU_DMC_FLAGS_RATE => self.dmc.set_rate(data),
            APU_DMC_DIRECT_LOAD => self.dmc.set_direct_load(data),
            APU_DMC_SAMPLE_ADDRESS => self.dmc.set_sample_address(data),
            APU_DMC_SAMPLE_LENGTH => self.dmc.set_sample_length(data),
            APU_STATUS => self.write_status(data),
            APU_FRAME_COUNTER => self.write_frame_counter(data),
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

    fn read_status(&mut self) -> u8 {
        let mut status = 0;
        if self.pulse_1.length_counter != 0 {
            status |= 1 << 0;
        }

        if self.pulse_2.length_counter != 0 {
            status |= 1 << 1;
        }

        if self.triangle.length_counter != 0 {
            status |= 1 << 2;
        }

        if self.noise.length_counter != 0 {
            status |= 1 << 3;
        }

        if self.dmc.remaining_bytes > 0 {
            status |= 1 << 4;
        }

        if self.frame_interrupt {
            status |= 1 << 6;
        }

        if self.dmc.interrupt {
            status |= 1 << 7;
        }

        self.frame_interrupt = false;
        
        status
    }

    fn write_status(&mut self, data: u8) {
        self.pulse_1.set_enabled((data & 0b01) > 0);
        self.pulse_2.set_enabled((data & 0b10) > 0);
        self.triangle.set_enabled((data & 0b100) > 0);
        self.noise.set_enabled((data & 0b1000) > 0);
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

        self.interrupt_inhibit = (data >> 6) & 0b1 > 0;
        if self.step_mode == 5 {
            self.clock_envelopes();
            self.clock_sweeps();
            self.triangle.clock_linear_counter();
            self.clock_length_counters();
        }
    }

    fn mix_samples(&mut self) -> f32 {
        let pulse_1 = self.pulse_1.clock();
        let pulse_2 = self.pulse_2.clock();
        let triangle = self.triangle.clock();
        let noise = self.noise.clock();
        let dmc = self.dmc.clock();

        let pulse_index = pulse_1 + pulse_2;
        let pulse_out = self.square_table[pulse_index as usize];
        let tnd_index = (3 * triangle) + (2 * noise) + dmc;
        let tnd_out = self.tnd_table[tnd_index as usize];

        pulse_out + tnd_out
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
                self.triangle.clock_linear_counter();
                self.clock_sweeps();
                self.clock_length_counters();

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
                self.triangle.clock_linear_counter();
                self.clock_length_counters();
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
                self.triangle.clock_linear_counter();
                self.clock_length_counters();
            },
            _ => ()
        }
    }

    fn clock_envelopes(&mut self) {
        self.pulse_1.envelope.clock();
        self.pulse_2.envelope.clock();
        self.noise.envelope.clock();
    }

    fn clock_sweeps(&mut self) {
        self.pulse_1.clock_sweep();
        self.pulse_2.clock_sweep();
    }

    fn clock_length_counters(&mut self) {
        self.pulse_1.clock_length_counter();
        self.pulse_2.clock_length_counter();
        self.triangle.clock_length_counter();
        self.noise.clock_length_counter();
    }

    fn is_max_step_counter(&self) -> bool {
        if self.step_mode == 4 {
            self.frame_clock_counter == 14915
        } else {
            self.frame_clock_counter == 18641
        }
    }
}