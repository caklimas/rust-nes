use crate::cpu;
use crate::ppu;
use crate::cartridge;
use crate::opcode_table;
use crate::address_modes;

const STACK_BASE_LOCATION: u16 = 0x0100;
const STACK_END_LOCATION: u8 = 0xFD;
const RESET_PROGRAM_COUNTER_ADDRESS: u16 = 0xFFFC;
const CPU_RAM_SIZE: usize = 2048;
const CPU_MAX_ADDRESS: u16 = 0x1FFF;
const CPU_MIRROR: u16 = 0x07FF;

pub struct Bus {
    pub cpu_ram: [u8; CPU_RAM_SIZE],
    pub cpu: cpu::Olc6502,
    pub ppu: ppu::Olc2C02,
    pub system_clock_counter: u32
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_ram: [0; CPU_RAM_SIZE],
            cpu: cpu::Olc6502::new(),
            ppu: ppu::Olc2C02::new(),
            system_clock_counter: 0
        }
    }

    pub fn load_cartridge(&mut self, cart: cartridge::Cartridge) {

    }

    pub fn reset(&mut self) {
        self.cpu.accumulator = 0;
        self.cpu.x_register = 0;
        self.cpu.y_register = 0;
        self.cpu.stack_pointer = STACK_END_LOCATION;

        self.cpu.program_counter = self.read_program_counter(RESET_PROGRAM_COUNTER_ADDRESS);
        self.cpu.addr_rel = 0x0000;
        self.cpu.addr_abs = 0x0000;
        self.cpu.fetched_data = 0x00;
        self.cpu.cycles = 0;
    }

    pub fn clock(&mut self) {
        if self.cpu.cycles == 0 {
            self.cpu.opcode = self.cpu_read(self.cpu.program_counter, false);
            self.cpu.program_counter += 1;

            let record = &opcode_table::OPCODE_TABLE[self.cpu.opcode as usize];
            self.cpu.cycles = record.4;
            
            let additional_cycle_1 = record.2(self);
            let additional_cycle_2 = record.1(self);

            self.cpu.cycles += additional_cycle_1 & additional_cycle_2;
        }

        self.cpu.cycles -= 1;
    }
    
    pub fn fetch(&mut self) -> u8 {
        self.cpu.fetched_data = match opcode_table::OPCODE_TABLE[self.cpu.opcode as usize].3 {
            address_modes::AddressMode::Imp => self.cpu.fetched_data,
            _ => self.cpu_read(self.cpu.addr_abs, false)
        };
        
        self.cpu.fetched_data
    }
    
    pub fn read_from_stack(&mut self) -> u8 {
        self.cpu.stack_pointer += 1;
        self.cpu_read(STACK_BASE_LOCATION + (self.cpu.stack_pointer as u16), false)
    }

    pub fn read_counter_from_stack(&mut self) -> u16 {
        let low = self.read_from_stack() as u16;
        let high = self.read_from_stack() as u16;
        (high << 8) | low
    }

    pub fn read_program_counter(&mut self, address: u16) -> u16 {
        let low = self.cpu_read(address, false) as u16;
        let high = self.cpu_read(address + 1, false) as u16;
        high << 8 | low
    }

    pub fn write_to_stack(&mut self, data: u8) {
        self.cpu_write(STACK_BASE_LOCATION + (self.cpu.stack_pointer as u16), data);
        self.cpu.stack_pointer -= 1;
    }
    
    pub fn write_counter_to_stack(&mut self) {
        self.write_to_stack(((self.cpu.program_counter >> 8) & 0x00FF) as u8);
        self.write_to_stack((self.cpu.program_counter & 0x00FF) as u8);
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        if address <= CPU_MAX_ADDRESS {
            self.cpu_ram[(address & CPU_MIRROR) as usize] = data;
        } else if address >= ppu::PPU_ADDRESS_START && address <= ppu::PPU_ADDRESS_END {
            self.ppu.cpu_write(address & ppu::PPU_ADDRESS_RANGE, data);
        }
    }

    pub fn cpu_read(&mut self, address: u16, read_only: bool) -> u8 {
        let mut data: u8 = 0x00;

        // Check the 8KB range of the CPU
        if address <= CPU_MAX_ADDRESS {
            // Need to mirror every 2KB
            data = self.cpu_ram[(address & CPU_MIRROR) as usize];
        } else if address >= ppu::PPU_ADDRESS_START && address <= ppu::PPU_ADDRESS_END {
            data = self.ppu.cpu_read(address & ppu::PPU_ADDRESS_RANGE, read_only);
        }

        data
    }
}