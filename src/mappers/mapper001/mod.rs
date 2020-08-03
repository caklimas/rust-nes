pub mod chr_bank;
pub mod control_register;
pub mod mapper001;
pub mod prg_bank;
pub mod shift_register;

const CHR_ROM_FIRST_BANK_LOWER: u16 = 0x0000;
const CHR_ROM_FIRST_BANK_UPPER: u16 = 0x0FFF;
const CHR_ROM_LAST_BANK_LOWER: u16 = 0x1000;
const CHR_ROM_LAST_BANK_UPPER: u16 = 0x1FFF;
const OPTIONAL_RAM_ADDRESS_LOWER: u16 = 0x6000;
const OPTIONAL_RAM_ADDRESS_UPPER: u16 = 0x7FFF;
const PRG_ROM_FIRST_BANK_LOWER: u16 = 0x8000;
const PRG_ROM_FIRST_BANK_UPPER: u16 = 0xBFFF;
const PRG_ROM_LAST_BANK_LOWER: u16 = 0xC000;
const PRG_ROM_LAST_BANK_UPPER: u16 = 0xFFFF;
const RAM_ADDRESS_MASK: u16 = 0x1FFF;

use crate::mappers::battery_backed_ram;
use crate::mappers::mapper::{Mapper};
use crate::mappers::mapper_results::{MapperReadResult, MapperWriteResult};
use crate::addresses::mappers::*;
use crate::memory_sizes::*;
use crate::cartridge::mirror::Mirror;

#[derive(Debug)]
pub struct Mapper001 {
    pub prg_banks: u8,
    pub chr_banks: u8,
    battery_backed_ram: bool,
    chr_bank: chr_bank::ChrBank,
    control_register: control_register::ControlRegister,
    prg_bank: prg_bank::PrgBank,
    ram: Vec<u8>,
    shift_register: shift_register::ShiftRegister
}

impl Mapper001 {
    pub fn new(prg_banks: u8, chr_banks: u8, battery_backed_ram: bool) -> Self {
        Mapper001 {
            prg_banks,
            chr_banks,
            battery_backed_ram,
            chr_bank: chr_bank::ChrBank::new(),
            control_register: control_register::ControlRegister(0x00),
            prg_bank: prg_bank::PrgBank::new(prg_banks),
            ram: vec![0; KILOBYTES_32 as usize],
            shift_register: Default::default()
        }
    }
}

impl Mapper for Mapper001 {
    fn reset(&mut self) {
        self.chr_bank.reset();
        self.control_register = control_register::ControlRegister(0x1C);
        self.prg_bank.reset();
        self.shift_register.reset();
    }

    fn get_prg_banks(&self) -> u8 {
        self.prg_banks
    }

    fn get_chr_banks(&self) -> u8 {
        self.chr_banks
    }

    fn get_mirror(&self) -> Mirror {
        self.control_register.get_mirror()
    }

    fn cpu_map_read(&self, address: u16) -> MapperReadResult {
        match address {
            OPTIONAL_RAM_ADDRESS_LOWER..=OPTIONAL_RAM_ADDRESS_UPPER => {
                let index = (address & RAM_ADDRESS_MASK) as usize;
                return MapperReadResult::from_mapper_ram(self.ram[index]);
            },
            PRG_ROM_FIRST_BANK_LOWER..=PRG_ROM_LAST_BANK_UPPER => {
                let mapped_address = self.prg_bank.get_mapped_address(address, &self.control_register.get_prg_mode());
                MapperReadResult::from_cart_ram(mapped_address)
            }
            _ => return MapperReadResult::none()
        }
    }

    fn cpu_map_write(&mut self, address: u16, data: u8) -> MapperWriteResult {
        if address < CPU_MIN_ADDRESS {
            match address {
                OPTIONAL_RAM_ADDRESS_LOWER..=OPTIONAL_RAM_ADDRESS_UPPER => {
                    let index = (address & RAM_ADDRESS_MASK) as usize;
                    self.ram[index] = data;
                    return MapperWriteResult::handled();
                },
                _ => return MapperWriteResult::none()
            }
        }

        let reset_shift = (data & 0b1000_0000) > 0;
        if reset_shift {
            self.reset_loading();
        } else {
            self.write(address, data);
        }

        MapperWriteResult::none()
    }

    fn ppu_map_read(&self, address: u16) -> MapperReadResult {
        if self.chr_banks == 0 && address <= CHR_ROM_LAST_BANK_UPPER {
            return MapperReadResult::from_cart_ram(address as u32)
        }

        match address {
            CHR_ROM_FIRST_BANK_LOWER..=CHR_ROM_LAST_BANK_UPPER => {
                let mapped_address = self.chr_bank.get_mapped_address(address, &self.control_register.get_chr_mode());
                MapperReadResult::from_cart_ram(mapped_address)
            },
            _ => MapperReadResult::none()
        }
    }

    fn ppu_map_write(&mut self, address: u16, mapped_address: &mut u32, _data: u8) -> bool {
        
        if address > PPU_MAX_ADDRESS || self.get_chr_banks() != 0 {
            return false;
        }
        
        *mapped_address = address as u32;
        true
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

impl Mapper001 {
    fn reset_loading(&mut self) {
        self.control_register.0 |= 0x0C;
        self.shift_register.reset_loading();
    }

    fn write(&mut self, address: u16, data: u8) {
        self.shift_register.push_data(data);
        if self.shift_register.bit_shift == 5
        {
            let address_select = (address & 0b110_0000_0000_0000) >> 13;
            match address_select {
                0 => self.control_register.write(self.shift_register.load_register),
                1 => self.chr_bank.write_low(&self.control_register.get_chr_mode(), self.shift_register.load_register),
                2 => self.chr_bank.write_high(&self.control_register.get_chr_mode(), self.shift_register.load_register),
                3 => self.prg_bank.write(&self.control_register.get_prg_mode(), self.shift_register.load_register),
                _ => panic!("Invalid address write Mapper001")
            }

            self.shift_register.reset_loading();
        }
    }
}