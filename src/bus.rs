use std::rc::Rc;
use std::cell::RefCell;

use crate::cpu::cpu;
use crate::ppu::ppu;
use crate::cartridge::cartridge;
use crate::memory;

pub struct Bus {
    pub cpu: cpu::Olc6502,
    pub ppu: Rc<RefCell<ppu::Olc2C02>>,
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub memory: Rc<RefCell<memory::Memory>>,
    pub system_clock_counter: u32,
    pub can_draw: bool,
    dma_dummy: bool
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
            system_clock_counter: 0,
            can_draw: false,
            dma_dummy: false
        }
    }

    pub fn load_cartridge(&mut self, cartridge: cartridge::Cartridge) {
        let c = Rc::new(RefCell::new(cartridge));
        self.cartridge = Some(Rc::clone(&c));
        self.ppu.borrow_mut().cartridge = Some(Rc::clone(&c));
        self.memory.borrow_mut().cartridge = Some(Rc::clone(&c));
        self.reset();
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
        self.ppu.borrow_mut().clock();

        // The CPU runs 3 times slower than the PPU
        if self.system_clock_counter % 3 == 0 {
            // If DMA transer is happening, then the cpu is suspended
            if self.memory.borrow().dma_transfer {
                // The DMA is synchronized with every other clock cycle
                // Without loss of generality, we will do it every odd cycle
                if self.dma_dummy && self.system_clock_counter % 2 == 1 {
                    self.dma_dummy = false;
                } else {
                    if self.system_clock_counter % 2 == 0 {
                        // Read data from cpu space
                        let dma = self.memory.borrow().dma;
                        let address = ((dma.page as u16) << 8) | (dma.address as u16);
                        let data = self.memory.borrow_mut().read(address, false);
                        self.memory.borrow_mut().dma.data = data;
                    } else {
                        // Write it to the ppu's OAM and increment DMA address
                        let dma = self.memory.borrow().dma;
                        self.ppu.borrow_mut().oam[dma.address as usize] = dma.data;
                        self.memory.borrow_mut().dma.address = dma.address.wrapping_add(1);

                        // Since we're wrapping around, we know when it goes back to zero that it has written all 256 bytes
                        if self.memory.borrow().dma.address == 0x00 {
                            self.memory.borrow_mut().dma_transfer = false;
                            self.dma_dummy = true;
                        }
                    }
                }
            } else {
                self.cpu.clock(self.system_clock_counter);
            }
        }

        if self.ppu.borrow_mut().nmi {
            self.ppu.borrow_mut().nmi = false;
            self.cpu.non_mask_interrupt_request();
        }

        self.system_clock_counter += 1;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.borrow_mut().reset();
        self.system_clock_counter = 0;
    }
}