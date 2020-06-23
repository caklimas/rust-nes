use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;
use crate::cpu;
use crate::ppu;
use crate::cartridge;

const CPU_MAX_ADDRESS: u16 = 0x1FFF;

pub struct Bus {
    pub cpu: cpu::Olc6502,
    pub ppu: ppu::Olc2C02,
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
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
        let c = Rc::new(RefCell::new(cartridge));
        self.cartridge = Some(Rc::clone(&c));
        self.ppu.cartridge = Some(Rc::clone(&c));
    }

    pub fn clock(&mut self) {
        // Clocking. The heart and soul of an emulator. The running
        // frequency is controlled by whatever calls this function.
        // So here we "divide" the clock as necessary and call
        // the peripheral devices clock() function at the correct
        // times

        // The fastest clock frequency the digital system cares
        // about is equivalent to the PPU clock. So the PPU is clocked
        // each time this function is called
        self.ppu.clock();

        // The CPU runs 3 times slower than the PPU
        if self.system_clock_counter % 3 == 0 {
            self.cpu.clock();
        }

        self.system_clock_counter += 1;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.system_clock_counter = 0;
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        if self.cartridge().borrow_mut().cpu_write(address, data) {
            return;
        }

        if address <= CPU_MAX_ADDRESS {
            self.cpu.write(address, data);
        } else if address >= ppu::PPU_ADDRESS_START && address <= ppu::PPU_ADDRESS_END {
            self.ppu.cpu_write(address & ppu::PPU_ADDRESS_RANGE, data);
        }
    }

    pub fn cpu_read(&mut self, address: u16, read_only: bool) -> u8 {
        let mut data: u8 = 0x00;
        if self.cartridge().borrow_mut().cpu_read(address, &mut data) {
            return data;
        }

        // Check the 8KB range of the CPU
        if address <= CPU_MAX_ADDRESS {
            // Need to mirror every 2KB
            data = self.cpu.read(address, read_only);
        } else if address >= ppu::PPU_ADDRESS_START && address <= ppu::PPU_ADDRESS_END {
            data = self.ppu.cpu_read(address & ppu::PPU_ADDRESS_RANGE, read_only);
        }

        data
    }

    fn cartridge(&mut self) -> Rc<RefCell<cartridge::Cartridge>> {
        self.cartridge.take().unwrap()
    }
}