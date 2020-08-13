use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct PrgRamProtect {
    enable_chip: bool,
    allow_writes: bool
}

impl PrgRamProtect {
    pub fn new() -> Self {
        PrgRamProtect {
            enable_chip: false,
            allow_writes: true
        }
    }

    pub fn set_data(&mut self, data: u8) {
        self.allow_writes = data & 0b0100_0000 == 0;
        self.enable_chip = data & 0b1000_0000 != 0;
    }
}