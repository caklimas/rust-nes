use std::rc::Rc;
use std::cell::RefCell;

use crate::ppu::ppu;
use crate::ppu::sprites;
use crate::cartridge::cartridge;
use crate::addresses;
use crate::controller;
use crate::audio;

const RAM_SIZE: usize = 2048;
const CPU_MAX_ADDRESS: u16 = 0x1FFF;
const CPU_MIRROR: u16 = 0x07FF;

pub struct Memory {
    ram: [u8; RAM_SIZE],
    ppu: Rc<RefCell<ppu::Ppu2C02>>,
    apu: Rc<RefCell<audio::apu::Apu>>,
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub controllers: [controller::Controller; 2],
    pub dma: sprites::DirectMemoryAccess,
    pub dma_transfer: bool
}

impl Memory {
    pub fn new(ppu: Rc<RefCell<ppu::Ppu2C02>>, apu: Rc<RefCell<audio::apu::Apu>>) -> Self {
        Memory {
            ram: [0; RAM_SIZE],
            ppu,
            apu,
            cartridge: None,
            controllers: Default::default(),
            dma: Default::default(),
            dma_transfer: false
        }
    }

    pub fn read(&mut self, address: u16, read_only: bool) -> u8 {
        let mut data: u8 = 0x00;

        match self.cartridge {
            Some(ref mut c) => {
                if c.borrow_mut().cpu_read(address, &mut data) {
                    return data;
                }
            },
            None => ()
        };
        
        // Check the 8KB range of the CPU
        if address <= CPU_MAX_ADDRESS {
            // Need to mirror every 2KB
            data = self.ram[(address & CPU_MIRROR) as usize];
        } else if address >= addresses::PPU_ADDRESS_START && address <= addresses::PPU_ADDRESS_END {
            data = self.ppu.borrow_mut().cpu_read(address & addresses::PPU_ADDRESS_RANGE, read_only);
        } else if address >= addresses::CONTROLLER_ONE_INPUT && address <= addresses::CONTROLLER_TWO_INPUT {
            let masked_address = address & 0x0001;
            data = self.controllers[masked_address as usize].get_msb();
        }

        data
    }
    
    pub fn write(&mut self, address: u16, data: u8) {
        match self.cartridge {
            Some(ref mut c) => {
                if c.borrow_mut().cpu_write(address, data) {
                    return;
                }
            },
            None => ()
        };

        if address <= CPU_MAX_ADDRESS {
            self.ram[(address & CPU_MIRROR) as usize] = data;
        } else if address >= addresses::PPU_ADDRESS_START && address <= addresses::PPU_ADDRESS_END {
            self.ppu.borrow_mut().cpu_write(address & addresses::PPU_ADDRESS_RANGE, data);
        } else if self.is_apu_address(address) {
            self.ppu.borrow_mut().cpu_write(address, data);
        } else if address == addresses::DMA_ADDRESS {
            self.dma.page = data;
            self.dma.address = 0x00;
            self.dma_transfer = true;
        } else if address >= addresses::CONTROLLER_ONE_INPUT && address <= addresses::CONTROLLER_TWO_INPUT {
            let masked_address = address & 0x0001;
            self.controllers[masked_address as usize].set_state();
        }
    }
    
    pub fn reset(&mut self) {
        for i in 0..self.ram.len() {
            self.ram[i] = 0
        }
    }

    fn is_apu_address(&mut self, address: u16) -> bool {
        (address >= addresses::APU_PULSE_1_TIMER && address <= addresses::APU_DMC) || address == addresses::APU_STATUS || address == addresses::APU_FRAME_COUNTER
    }
}