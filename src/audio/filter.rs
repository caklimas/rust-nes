use std::f32::consts::PI;

pub struct Filter {
    gamma: f32,
    previous_input: f32,
    previous_output: f32
}

impl Filter {
    pub fn new(cutoff_frequency: f32, coefficient: Coefficient) -> Self {
        let gamma = match coefficient {
            Coefficient::High => high_pass_coefficient(cutoff_frequency),
            Coefficient::Low => low_pass_coefficient(cutoff_frequency)
        };

        Filter {
            gamma, 
            previous_input: 0.0,
            previous_output: 0.0
        }
    }

    pub fn high_pass(&self, sample: f32) -> f32 {
        (self.gamma * self.previous_output) + (sample - self.previous_input)
    }

    pub fn low_pass(&self, sample: f32) -> f32 {
        ((1.0 - self.gamma) * self.previous_output) + (self.gamma * sample)
    }
}

pub enum Coefficient {
    High,
    Low
}

fn high_pass_coefficient(cutoff_frequency: f32) -> f32 {
    1.0 / (calculate_frequency(cutoff_frequency) + 1.0)
}

fn low_pass_coefficient(cutoff_frequency: f32) -> f32 {
    let frequency = calculate_frequency(cutoff_frequency);
    frequency / (frequency + 1.0)
}

fn calculate_frequency(cutoff_frequency: f32) -> f32 {
    2.0 * PI * cutoff_frequency / super::SAMPLE_RATE as f32
}