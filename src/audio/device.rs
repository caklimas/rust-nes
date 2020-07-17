use std::sync::{Arc, Mutex};
use sdl2::audio::{AudioCallback, AudioSpecDesired};

const SAMPLE_RATE: i32 = 44_100;

pub struct AudioDevice {
    pub buffer: Arc<Mutex<Vec<f32>>>
}

impl AudioDevice {
    pub fn new(sdl_context: &sdl2::Sdl, buffer: Arc<Mutex<Vec<f32>>>) -> sdl2::audio::AudioDevice<AudioDevice> {
        let audio_subsystem = sdl_context.audio().expect("Error loading audio subsystem");
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
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
        let mut b = self.buffer.lock().expect("Error retrieving buffer");
        for (i, x) in out.iter_mut().enumerate() {
            if i < b.len() {
                *x = b[i];
            }
        }

        b.clear();
    }
}