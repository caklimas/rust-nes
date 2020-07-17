use sdl2::audio::{AudioSpecDesired, AudioSpecWAV, AudioCallback, AudioCVT};
use std::time::Duration;

pub fn play_sound() {
    let sdl_context = sdl2::init().expect("Error loading sdl2 context");
    let audio_subsystem = sdl_context.audio().expect("Error loading audio subsystem");

    let desired_audio_spec = AudioSpecDesired {
        freq: Some(44_100),
        // Mono
        channels: Some(1),
        // Doesn't matter here, use the default value
        samples: None
    };

    let mut callback = SimpleCallback {
        buffer: Vec::new(),
        position: 0,
    };
    
    let mut audio_device = audio_subsystem
        .open_playback(None, &desired_audio_spec, |spec| {
            let wav = AudioSpecWAV::load_wav(r"C:\Users\Christopher\Downloads\beep-01a.wav").expect("Error loading WAV");
            let converter = AudioCVT::new(
                wav.format,
                wav.channels,
                wav.freq,
                spec.format,
                spec.channels,
                spec.freq,
            )
            .unwrap();
            let data = converter.convert(wav.buffer().to_vec());
            callback.buffer = data;
            callback
        })
        .unwrap();
    
    // This starts the playback.
    audio_device.resume();
    
    std::thread::sleep(Duration::from_millis(1_000));
    {
        let mut lock = audio_device.lock();
        lock.position = 0;
    }
    std::thread::sleep(Duration::from_millis(1_000));
}

struct SimpleCallback {
    buffer: Vec<u8>,
    position: usize
}

impl AudioCallback for SimpleCallback {
    type Channel = i16;

    fn callback(&mut self, out: &mut[Self::Channel]) {
        for value in out.iter_mut() {
            *value = if self.position < self.buffer.len() {
                let sample = i16::from_ne_bytes([
                    self.buffer[self.position],
                    self.buffer[self.position + 1]
                ]);

                self.position += 2;
                sample
            } else {
                0
            }
        }
    }
}