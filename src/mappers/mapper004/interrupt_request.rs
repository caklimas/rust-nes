use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct InterruptRequest {
    pub active: bool,
    pub counter: u8,
    pub latch: u8,
    enabled: bool
}

impl InterruptRequest {
    pub fn new() -> Self {
        InterruptRequest {
            active: false,
            counter: 0,
            enabled: false,
            latch: 0
        }
    }

    pub fn clock(&mut self) {
        if self.counter == 0 {
            self.counter = self.latch;
        } else {
            self.counter -= 1;
        }

        if self.counter == 0 && self.enabled {
            self.active = true;
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.active = false;
        }
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.counter = 0;
        self.enabled = false;
        self.latch = 0;
    }
}