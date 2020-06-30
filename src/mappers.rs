use crate::memory_sizes;

const CPU_MIN_ADDRESS: u16 = 0x8000;
const PPU_MAX_ADDRESS: u16 = 0x1FFF;

pub trait Mapper {
    fn get_prg_banks(&mut self) -> u8;
    fn get_chr_banks(&mut self) -> u8;
    fn cpu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool;
    fn cpu_map_write(&mut self, address: u16, mapped_address: &mut u32) -> bool;
    fn ppu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool;
    fn ppu_map_write(&mut self, address: u16, mapped_address: &mut u32) -> bool;
}

pub struct Mapper000 {
    pub prg_banks: u8,
    pub chr_banks: u8
}

impl Mapper for Mapper000 {
    fn get_prg_banks(&mut self) -> u8 {
        self.prg_banks
    }

    fn get_chr_banks(&mut self) -> u8 {
        self.chr_banks
    }

    fn cpu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool {
        if address < CPU_MIN_ADDRESS {
            return false;
        }
        
        let prg_banks = self.get_prg_banks();
        let masked_address = if prg_banks > 1 { memory_sizes::KILOBYTES_32 } else { memory_sizes::KILOBYTES_16 };
        *mapped_address = (address & masked_address) as u32;

        true
    }

    fn cpu_map_write(&mut self, address: u16, mapped_address: &mut u32) -> bool {
        if address < CPU_MIN_ADDRESS {
            return false;
        }

        let prg_banks = self.get_prg_banks();
        let masked_address = if prg_banks > 1 { memory_sizes::KILOBYTES_32 } else { memory_sizes::KILOBYTES_16 };
        *mapped_address = (address & masked_address) as u32;

        true
    }

    fn ppu_map_read(&mut self, address: u16, mapped_address: &mut u32) -> bool {
        if address > PPU_MAX_ADDRESS {
            return false;
        }

        *mapped_address = address as u32;
        true
    }

    fn ppu_map_write(&mut self, _address: u16, _mapped_address: &mut u32) -> bool {
        false
    }
}
