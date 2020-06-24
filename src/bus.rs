use std::rc::Rc;
use std::cell::RefCell;

use crate::cpu;
use crate::ppu;
use crate::cartridge;
use crate::memory;

const CPU_MAX_ADDRESS: u16 = 0x1FFF;

pub struct Bus {
    pub cpu: cpu::Olc6502,
    pub ppu: Rc<RefCell<ppu::Olc2C02>>,
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub memory: Rc<RefCell<memory::Memory>>,
    pub system_clock_counter: u32
}

impl Bus {
    pub fn new() -> Self {
        let ppu = Rc::new(RefCell::new(ppu::Olc2C02::new()));
        let memory = Rc::new(RefCell::new(memory::Memory::new(Rc::clone(&ppu))));

        Bus {
            cpu: cpu::Olc6502::new(Rc::clone(&memory)),
            ppu: Rc::clone(&ppu),
            cartridge: None,
            memory: Rc::clone(&memory),
            system_clock_counter: 0
        }
    }

    pub fn load_cartridge(&mut self, cartridge: cartridge::Cartridge) {
        let c = Rc::new(RefCell::new(cartridge));
        self.cartridge = Some(Rc::clone(&c));
        self.ppu.borrow_mut().cartridge = Some(Rc::clone(&c));
        self.memory.borrow_mut().load_cartridge(c);
        self.cpu.reset();
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
        // self.ppu.clock();

        // The CPU runs 3 times slower than the PPU
        if self.system_clock_counter % 3 == 0 {
            self.cpu.clock();
        }

        self.system_clock_counter += 1;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.borrow_mut().reset();
        self.system_clock_counter = 0;
    }
}