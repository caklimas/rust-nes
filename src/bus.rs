use std::rc::Rc;
use std::cell::RefCell;

use crate::addresses::{AddressRange, get_address_range};
use crate::ppu::ppu;
use crate::ppu::sprites;
use crate::cartridge;
use crate::audio;
use crate::controller;

const RAM_SIZE: usize = 2048;
const CPU_MIRROR: u16 = 0x07FF;

pub struct Bus {
    pub ppu: ppu::Ppu2C02,
    pub apu: audio::Apu2A03,
    pub cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub controllers: [controller::Controller; 2],
    pub dma: sprites::DirectMemoryAccess,
    pub dma_transfer: bool,
    pub audio_sample: f64,
    pub strobe_pulse: u8,
    ram: [u8; RAM_SIZE]
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ppu: ppu::Ppu2C02::new(),
            apu: audio::Apu2A03::initialize(),
            cartridge: None,
            controllers: Default::default(),
            dma: Default::default(),
            dma_transfer: false,
            audio_sample: 0.0,
            strobe_pulse: 0,
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
        if let Some(ref mut c) = self.cartridge {
            if c.borrow_mut().cpu_read(address, &mut data) {
                return data;
            }
        }

        data = match get_address_range(address) {
            AddressRange::Cpu => self.ram[(address & CPU_MIRROR) as usize],
            AddressRange::Ppu => self.ppu.read(address),
            AddressRange::Dma => 0,
            AddressRange::Apu => self.apu.read(address),
            AddressRange::Controller => self.read_controllers(address),
            AddressRange::Unknown => 0
        };

        data
    }

    pub fn write(&mut self, address: u16, data: u8) {
        if let Some(ref mut c) = self.cartridge {
            if c.borrow_mut().cpu_write(address, data) {
                return;
            }
        }

        match get_address_range(address) {
            AddressRange::Cpu => self.ram[(address & CPU_MIRROR) as usize] = data,
            AddressRange::Ppu => self.ppu.write(address, data),
            AddressRange::Dma => self.write_dma(data),
            AddressRange::Apu => self.apu.write(address, data),
            AddressRange::Controller => self.write_controllers(address, data),
            AddressRange::Unknown => ()
        }
    }

    pub fn reset(&mut self) {
        self.apu.reset();
        if let Some(ref mut c) = self.cartridge {
            c.borrow_mut().reset();
        }

        for i in 0..self.ram.len() {
            self.ram[i] = 0
        }
    }

    fn read_controllers(&mut self, address: u16) -> u8 {
        let masked_address = address & 0x0001;
        self.controllers[masked_address as usize].read(&self.strobe_pulse)
    }

    fn write_dma(&mut self, data: u8) {
        self.dma.page = data;
        self.dma.address = 0x00;
        self.dma_transfer = true;
    }

    fn write_controllers(&mut self, address: u16, data: u8) {
        self.strobe_pulse = data;
        let masked_address = address & 0x0001;
        self.controllers[masked_address as usize].write(&self.strobe_pulse)
    }
}