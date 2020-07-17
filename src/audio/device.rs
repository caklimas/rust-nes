use std::sync::{Arc, Mutex};
use sdl2::audio::{AudioCallback, AudioSpecDesired};

pub struct AudioDevice {
    pub buffer: Arc<Mutex<Vec<f32>>>
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
    }
}