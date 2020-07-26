use std::sync::{Arc, Mutex};
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use super::filter::{Filter, Coefficient};

const CPU_SAMPLE_RATE: f32 = 1_789_773.0;
const APU_SAMPLE_RATE: f32 = CPU_SAMPLE_RATE / 2.0;
const SAMPLES: u16 = (super::SAMPLE_RATE as u16) / 60;
const SAMPLE_RATIO: f32 = APU_SAMPLE_RATE / (super::SAMPLE_RATE as f32);

pub struct AudioDevice {
    pub buffer: Arc<Mutex<Vec<f32>>>,
    filter_90: Filter,
    filter_440: Filter,
    filter_14000: Filter
}

impl AudioDevice {
    pub fn new(sdl_context: &sdl2::Sdl, buffer: Arc<Mutex<Vec<f32>>>) -> sdl2::audio::AudioDevice<AudioDevice> {
        let audio_subsystem = sdl_context.audio().expect("Error loading audio subsystem");
        let desired_spec = AudioSpecDesired {
            freq: Some(super::SAMPLE_RATE),
            channels: Some(1),  // mono
            samples: Some(SAMPLES)
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |_spec| {
            // initialize the audio callback
            AudioDevice {
                buffer,
                filter_90: Filter::new(90.0, Coefficient::High),
                filter_440: Filter::new(440.0, Coefficient::High),
                filter_14000: Filter::new(14_000.0, Coefficient::Low)
            }
        }).expect("Error opening device");

        device
    }
}

impl AudioCallback for AudioDevice {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        let mut lock = self.buffer.lock().expect("Error retrieving buffer");
        if lock.len() > 0 {
            for (i, x) in out.iter_mut().enumerate() {
                let sample_index = ((i as f32) * SAMPLE_RATIO) as usize;
                if sample_index < lock.len() {
                    let sample = lock[sample_index];
                    let filtered_90 = self.filter_90.high_pass(sample);
                    let filtered_440 = self.filter_440.high_pass(filtered_90);
                    let filtered_14_000 = self.filter_14000.low_pass(filtered_440);

                    *x = filtered_14_000;
                }
            }

            let target_index = (SAMPLES as f32 * SAMPLE_RATIO) as usize;
            if lock.len() > target_index {
                *lock = lock.split_off(target_index);
            }
        }
    }
}