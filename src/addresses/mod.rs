pub mod apu;
pub mod cpu;
pub mod controllers;
pub mod dma;
pub mod ppu;

use apu::*;
use controllers::*;
use cpu::*;
use dma::*;
use ppu::*;

pub fn get_address_range(address: u16) -> AddressRange {
    match address {
        0..=CPU_ADDRESS_UPPER => AddressRange::Cpu,
        PPU_ADDRESS_START..=PPU_ADDRESS_END => AddressRange::Ppu,
        DMA_ADDRESS => AddressRange::Dma,
        APU_PULSE_1_DUTY..=APU_DMC_SAMPLE_LENGTH | APU_STATUS | APU_FRAME_COUNTER => AddressRange::Apu,
        CONTROLLER_ONE_INPUT => AddressRange::Controller,
        _ => AddressRange::Unknown
    }
}

pub enum AddressRange {
    Cpu,
    Ppu,
    Dma,
    Apu,
    Controller,
    Unknown
}