use crate::cpu;

const RAM_SIZE: usize = 64 * 1024;

pub struct Bus {
    pub cpu: cpu::olc6502,
    pub ram: [u8; RAM_SIZE]
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu: cpu::olc6502::new(),
            ram: [0; RAM_SIZE]
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }

    pub fn read(&mut self, address: u16, is_read_only: bool) -> u8 {
        return self.ram[address as usize];
    }
}