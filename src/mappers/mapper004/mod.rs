pub mod bank_select;
pub mod prg_ram_protect;
pub mod interrupt_request;

use crate::mappers::battery_backed_ram;
use super::mapper::{Mapper};
use super::mapper_results::{MapperReadResult, MapperWriteResult};
use crate::memory_sizes::*;
use crate::cartridge::mirror::Mirror;

const OPTIONAL_RAM_ADDRESS_LOWER: u16 = 0x6000;
const OPTIONAL_RAM_ADDRESS_UPPER: u16 = 0x7FFF;
const RAM_ADDRESS_MASK: u16 = 0x1FFF;

pub struct Mapper004 {
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub interrupt_request: interrupt_request::InterruptRequest,
    bank_select: bank_select::BankSelect,
    battery_backed_ram: bool,
    mirror: Mirror,
    prg_ram_protect: prg_ram_protect::PrgRamProtect,
    ram: Vec<u8>
}

impl Mapper004 {
    pub fn new(prg_banks: u8, chr_banks: u8, battery_backed_ram: bool) -> Self {
        Mapper004 {
            prg_banks,
            chr_banks,
            bank_select: bank_select::BankSelect::new(prg_banks),
            battery_backed_ram,
            interrupt_request: interrupt_request::InterruptRequest::new(),
            mirror: Mirror::Horizontal,
            prg_ram_protect: prg_ram_protect::PrgRamProtect::new(),
            ram: vec![0; KILOBYTES_8 as usize]
        }
    }
}

impl Mapper for Mapper004 {
    fn reset(&mut self) {
        self.mirror = Mirror::Horizontal;
        self.bank_select.reset();
        self.interrupt_request.reset();
    }

    fn get_prg_banks(&self) -> u8 {
        self.prg_banks
    }

    fn get_chr_banks(&self) -> u8 {
        self.chr_banks
    }

    fn get_mirror(&self) -> Mirror {
        self.mirror
    }

    fn irq_active(&self) -> bool { 
        self.interrupt_request.active
    }

    fn irq_clear(&mut self) {
        self.interrupt_request.active = false;
    }

    fn irq_scanline(&mut self) {
        self.interrupt_request.clock();
    }

    fn cpu_map_read(&self, address: u16) -> MapperReadResult {
        match address {
            OPTIONAL_RAM_ADDRESS_LOWER..=OPTIONAL_RAM_ADDRESS_UPPER => {
                let index = (address & RAM_ADDRESS_MASK) as usize;
                return MapperReadResult::from_mapper_ram(self.ram[index]);
            },
            _ => ()
        };

        let address_offset = (address & KILOBYTES_8_MASK) as u32;
        match address {
            0x8000..=0x9FFF => {
                let mapped_address = self.bank_select.prg_banks[0] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address as u32)
            },
            0xA000..=0xBFFF => {
                let mapped_address = self.bank_select.prg_banks[1] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address as u32)
            },
            0xC000..=0xDFFF => {
                let mapped_address = self.bank_select.prg_banks[2] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address as u32)
            },
            0xE000..=0xFFFF => {
                let mapped_address = self.bank_select.prg_banks[3] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address as u32)
            },
            _ => MapperReadResult::none()
        }
    }

    fn cpu_map_write(&mut self, address: u16, data: u8) -> MapperWriteResult {
        match address {
            OPTIONAL_RAM_ADDRESS_LOWER..=OPTIONAL_RAM_ADDRESS_UPPER => {
                let index = (address & RAM_ADDRESS_MASK) as usize;
                self.ram[index] = data;
                return MapperWriteResult::handled();
            },
            _ => ()
        }

        match address {
            0x8000..=0x9FFF => {
                if address % 2 == 0 {
                    self.bank_select.select_bank(data);
                } else {
                    self.bank_select.set_bank_data(data);
                }
            },
            0xA000..=0xBFFF => {
                if address % 2 == 0 {
                    self.set_mirror(data);
                } else {
                    self.prg_ram_protect.set_data(data);
                }
            },
            0xC000..=0xDFFF => {
                if address % 2 == 0 {
                    self.interrupt_request.latch = data;
                } else {
                    self.interrupt_request.counter = 0;
                }
            },
            0xE000..=0xFFFF => {
                self.interrupt_request.set_enabled(address % 2 == 1);
            },
            _ => ()
        }

        MapperWriteResult::none()
    }

    fn ppu_map_read(&self, address: u16) -> MapperReadResult {
        let address_offset = (address & KILOBYTES_1_MASK) as u32;
        match address {
            0x0000..=0x03FF => {
                let mapped_address = self.bank_select.chr_banks[0] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address)
            },
            0x0400..=0x07FF => {
                let mapped_address = self.bank_select.chr_banks[1] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address)
            },
            0x0800..=0x0BFF => {
                let mapped_address = self.bank_select.chr_banks[2] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address)
            },
            0x0C00..=0x0FFF => {
                let mapped_address = self.bank_select.chr_banks[3] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address)
            },
            0x1000..=0x13FF => {
                let mapped_address = self.bank_select.chr_banks[4] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address)
            },
            0x1400..=0x17FF => {
                let mapped_address = self.bank_select.chr_banks[5] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address)
            },
            0x1800..=0x1BFF => {
                let mapped_address = self.bank_select.chr_banks[6] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address)
            },
            0x1C00..=0x1FFF => {
                let mapped_address = self.bank_select.chr_banks[7] + address_offset;
                MapperReadResult::from_cart_ram(mapped_address)
            },
            _ => MapperReadResult::none()
        }
    }

    fn ppu_map_write(&mut self, _address: u16, _mapped_address: &mut u32, _data: u8) -> bool {
        return false;
    }

    fn load_battery_backed_ram(&mut self, data: Vec<u8>) {
        if !self.battery_backed_ram {
            return;
        }

        self.ram = data;
    }

    fn save_battery_backed_ram(&self, file_path: &str) {
        if !self.battery_backed_ram {
            return;
        }
        
        battery_backed_ram::save_battery_backed_ram(file_path, &self.ram);
    }
}

impl Mapper004 {
    pub fn set_mirror(&mut self, data: u8) {
        self.mirror = if data & 0b1 == 0 {
            Mirror::Vertical
        } else {
            Mirror::Horizontal
        };
    }
}