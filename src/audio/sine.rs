use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

pub fn play_sound() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        println!("{:?}", spec);

        // initialize the audio callback
        SineWave {
            time: 0.0,
            freq: 440.0
        }
    }).unwrap();

    // Start playback
    device.resume();

    // Play for 2 seconds
    std::thread::sleep(Duration::from_millis(2000));
}

struct SineWave {
    time: f32,
    freq: f32
}

impl AudioCallback for SineWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = 32000.0 * self.time.sin();
            self.time += ((self.freq as f64) * std::f64::consts::PI) as f32;

            if self.time >= (std::f64::consts::PI as f32) {
                self.time -= std::f64::consts::PI as f32;
            }
        }
    }
}