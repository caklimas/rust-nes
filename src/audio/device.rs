use std::sync::{Arc, Mutex};
use sdl2::audio::{AudioCallback, AudioSpecDesired};

const CPU_SAMPLE_RATE: f32 = 1_789_773.0;
const APU_SAMPLE_RATE: f32 = CPU_SAMPLE_RATE / 2.0;
const SAMPLES: u16 = (super::SAMPLE_RATE as u16) / 60;
const SAMPLE_RATIO: f32 = APU_SAMPLE_RATE / (super::SAMPLE_RATE as f32);

pub struct AudioDevice {
    pub buffer: Arc<Mutex<Vec<f32>>>
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
                buffer
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
                    *x = lock[sample_index];
                }
            }

            let target_index = (SAMPLES as f32 * SAMPLE_RATIO) as usize;
            if lock.len() > target_index {
                *lock = lock.split_off(target_index);
            }
        }
    }
}