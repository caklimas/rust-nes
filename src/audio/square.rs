use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

pub fn play_sound(frequency: f32) {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        SquareWave {
            frequency,
            sample: 0,
            sample_rate: 44100,
            volume: 0.05
        }
    }).unwrap();

    // Start playback
    device.resume();

    // Play for 2 seconds
    std::thread::sleep(Duration::from_millis(2000));
}

struct SquareWave {
    frequency: f32,
    sample: usize,
    sample_rate: i32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            self.sample = self.sample.wrapping_add(1);
            let value = super::get_angular_frequency(self.frequency) * self.sample as f32 / self.sample_rate as f32;
            *x = if value.sin() > 0.0 {
                self.volume
            } else {
                -self.volume
            };
        }
    }
}