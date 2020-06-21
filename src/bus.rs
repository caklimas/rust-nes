use crate::cpu;
use crate::ppu;
use crate::cartridge;

const CPU_MAX_ADDRESS: u16 = 0x1FFF;

pub struct Bus {
    pub cpu: cpu::Olc6502,
    pub ppu: ppu::Olc2C02,
    pub cartridge: Option<cartridge::Cartridge>,
    pub system_clock_counter: u32
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu: cpu::Olc6502::new(),
            ppu: ppu::Olc2C02::new(),
            cartridge: None,
            system_clock_counter: 0
        }
    }

    pub fn load_cartridge(&mut self, cartridge: cartridge::Cartridge) {
        self.cartridge = Some(cartridge);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        if address <= CPU_MAX_ADDRESS {
            self.cpu.write(address, data);
        } else if address >= ppu::PPU_ADDRESS_START && address <= ppu::PPU_ADDRESS_END {
            self.ppu.cpu_write(address & ppu::PPU_ADDRESS_RANGE, data);
        }
    }

    pub fn cpu_read(&mut self, address: u16, read_only: bool) -> u8 {
        let mut data: u8 = 0x00;

        // Check the 8KB range of the CPU
        if address <= CPU_MAX_ADDRESS {
            // Need to mirror every 2KB
            data = self.cpu.read(address, read_only);
        } else if address >= ppu::PPU_ADDRESS_START && address <= ppu::PPU_ADDRESS_END {
            data = self.ppu.cpu_read(address & ppu::PPU_ADDRESS_RANGE, read_only);
        }

        data
    }
}