use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};
use crate::instant::InstantWrapper;

#[derive(Serialize, Deserialize)]
pub struct FpsLimiter {
    fps: u8,
    frames: u64,
    #[serde(skip_serializing, skip_deserializing)]
    fps_timer: InstantWrapper
}

impl FpsLimiter {
    pub fn new(fps: u8) -> Self {
        FpsLimiter {
            fps,
            frames: 0,
            fps_timer: Default::default()
        }
    }
    
    pub fn calculate_fps(&mut self) {
        let now = Instant::now();
        if now > self.fps_timer.instant + Duration::from_secs(1) {
            self.frames = 0;
            self.fps_timer.instant = now;
        }
    }

    pub fn limit(&mut self, timer: &InstantWrapper) {
        self.frames += 1;
        let now = Instant::now();
        let milliseconds: u64 = (1000 as u64) / (self.fps as u64);
        if now < timer.instant + Duration::from_millis(milliseconds) {
            std::thread::sleep(timer.instant + Duration::from_millis(milliseconds) - now);
        }
    }
}