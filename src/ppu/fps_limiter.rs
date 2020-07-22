use std::time::{Duration, Instant};

pub struct FpsLimiter {
    fps: u8,
    frames: u64,
    fps_timer: Instant
}

impl FpsLimiter {
    pub fn new(fps: u8) -> Self {
        FpsLimiter {
            fps,
            frames: 0,
            fps_timer: Instant::now()
        }
    }
    
    pub fn calculate_fps(&mut self) {
        let now = Instant::now();
        if now > self.fps_timer + Duration::from_secs(1) {
            println!("frames per second limiter: {}", self.frames);
            self.frames = 0;
            self.fps_timer = now;
        }
    }

    pub fn limit(&mut self, timer: Instant) {
        self.frames += 1;
        let now = Instant::now();
        let milliseconds: u64 = (1000 as u64) / (self.fps as u64);
        if now < timer + Duration::from_millis(milliseconds) {
            std::thread::sleep(timer + Duration::from_millis(milliseconds) - now);
        }
    }
}