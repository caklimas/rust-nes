use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;

use crate::addresses;
use crate::ppu::ppu;
use crate::ppu::sprites;
use crate::cartridge::cartridge;
use crate::audio;
use crate::controller;

const RAM_SIZE: usize = 2048;
const CPU_MIRROR: u16 = 0x07FF;

pub struct Bus {
    pub ppu: ppu::Ppu2C02,
    pub apu: audio::apu::Apu2A03,
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub controllers: [controller::Controller; 2],
    pub dma: sprites::DirectMemoryAccess,
    pub dma_transfer: bool,
    pub audio_sample: f64,
    ram: [u8; RAM_SIZE]
}

impl Bus {
    pub fn new(buffer: Arc<Mutex<Vec<f32>>>) -> Self {
        Bus {
            ppu: ppu::Ppu2C02::new(),
            apu: audio::apu::Apu2A03::initialize(),
            cartridge: None,
            controllers: Default::default(),
            dma: Default::default(),
            dma_transfer: false,
            audio_sample: 0.0,
            ram: [0; RAM_SIZE]
        }
    }

    pub fn load_cartridge(&mut self, cartridge: cartridge::Cartridge) {
        let c = Rc::new(RefCell::new(cartridge));
        self.cartridge = Some(Rc::clone(&c));
        self.ppu.cartridge = Some(Rc::clone(&c));
        self.reset();
    }

    pub fn read(&mut self, address: u16) -> u8 {
        let mut data: u8 = 0;

        match self.cartridge {
            Some(ref mut c) => {
                if c.borrow_mut().cpu_read(address, &mut data) {
                    return data;
                }
            },
            None => ()
        };

        if address <= addresses::CPU_ADDRESS_UPPER {
            data = self.ram[(address & CPU_MIRROR) as usize];
        } else if address >= addresses::PPU_ADDRESS_START && address <= addresses::PPU_ADDRESS_END {
            data = self.ppu.read(address & addresses::PPU_ADDRESS_RANGE);
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

        if address <= addresses::CPU_ADDRESS_UPPER {
            self.ram[(address & CPU_MIRROR) as usize] = data;
        } else if address >= addresses::PPU_ADDRESS_START && address <= addresses::PPU_ADDRESS_END {
            self.ppu.write(address & addresses::PPU_ADDRESS_RANGE, data);
        } else if address == addresses::DMA_ADDRESS {
            self.dma.page = data;
            self.dma.address = 0x00;
            self.dma_transfer = true;
        } else if self.is_apu_address(address) {
            self.apu.write(address, data);
        } else if address >= addresses::CONTROLLER_ONE_INPUT && address <= addresses::CONTROLLER_TWO_INPUT {
            let masked_address = address & 0x0001;
            self.controllers[masked_address as usize].set_state();
        }
    }

    pub fn reset(&mut self) {
        self.apu.reset();
        for i in 0..self.ram.len() {
            self.ram[i] = 0
        }
    }

    fn is_apu_address(&mut self, address: u16) -> bool {
        (address >= addresses::APU_PULSE_1_DUTY && address <= addresses::APU_DMC) || address == addresses::APU_STATUS || address == addresses::APU_FRAME_COUNTER
    }
}