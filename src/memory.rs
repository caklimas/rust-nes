use std::rc::Rc;
use std::cell::RefCell;

use crate::ppu;
use crate::cartridge;
use crate::addresses;

const RAM_SIZE: usize = 2048;
const CPU_MAX_ADDRESS: u16 = 0x1FFF;
const CPU_MIRROR: u16 = 0x07FF;

pub struct Memory {
    ram: [u8; RAM_SIZE],
    ppu: Rc<RefCell<ppu::Olc2C02>>,
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>
}

impl Memory {
    pub fn new(ppu: Rc<RefCell<ppu::Olc2C02>>) -> Self {
        Memory {
            ram: [0; RAM_SIZE],
            ppu: ppu,
            cartridge: None
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
        }
    }
    
    pub fn reset(&mut self) {
        for i in 0..self.ram.len() {
            self.ram[i] = 0
        }
    }
}