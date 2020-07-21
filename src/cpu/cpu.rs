use crate::bus;
use crate::cpu::opcode_table;
use crate::cpu::address_modes;

const NON_MASK_INTERRUPT_PROGRAM_COUNTER_ADDRESS: u16 = 0xFFFA;
const RESET_PROGRAM_COUNTER_ADDRESS: u16 = 0xFFFC;
const STACK_BASE_LOCATION: u16 = 0x0100;
const STACK_END_LOCATION: u8 = 0xFD;

pub const INTERRUPT_PROGRAM_COUNTER_ADDRESS: u16 = 0xFFFE;

pub struct Cpu6502 {
    pub bus: bus::Bus,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub status_register: u8,
    pub fetched_data: u8,
    pub addr_abs: u16,
    pub addr_rel: u16,
    pub opcode: u8,
    pub cycles: u8
}

impl Cpu6502 {
    pub fn new() -> Self {
        Cpu6502 {
            bus: bus::Bus::new(),
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: STACK_END_LOCATION,
            program_counter: 0,
            status_register: 0x24,
            fetched_data: 0,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0
        }
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(self.program_counter);
            let record = &opcode_table::OPCODE_TABLE[self.opcode as usize];
            self.program_counter = self.program_counter.wrapping_add(1);

            self.cycles = record.4;
            
            let additional_cycle_1 = record.2(self);
            let additional_cycle_2 = record.1(self);

            self.cycles += additional_cycle_1 & additional_cycle_2;
        }

        if self.cycles > 0 {
            self.cycles -= 1;
        }
    }
    
    pub fn fetch(&mut self) -> u8 {
        self.fetched_data = match opcode_table::OPCODE_TABLE[self.opcode as usize].3 {
            address_modes::AddressMode::Imp => self.fetched_data,
            _ => self.read(self.addr_abs)
        };
        
        self.fetched_data
    }

    pub fn reset(&mut self) {
        self.bus.reset();
        self.accumulator = 0;
        self.x_register = 0;
        self.y_register = 0;
        self.stack_pointer = STACK_END_LOCATION;

        self.program_counter = self.read_program_counter(RESET_PROGRAM_COUNTER_ADDRESS);
        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched_data = 0x00;

        // Resetting takes time
        self.cycles = 8;
    }

    pub fn read(&mut self, address: u16) -> u8 {
        self.bus.read(address)
    }

    pub fn read_from_stack(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.read(STACK_BASE_LOCATION + (self.stack_pointer as u16),)
    }

    pub fn read_counter_from_stack(&mut self) -> u16 {
        let low = self.read_from_stack() as u16;
        let high = self.read_from_stack() as u16;
        (high << 8) | low
    }

    pub fn read_program_counter(&mut self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address + 1) as u16;
        high << 8 | low
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.bus.write(address, data);
    }
    
    pub fn write_to_stack(&mut self, data: u8) {
        self.write(STACK_BASE_LOCATION + (self.stack_pointer as u16), data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
    
    pub fn write_counter_to_stack(&mut self) {
        self.write_to_stack(((self.program_counter >> 8) & 0x00FF) as u8);
        self.write_to_stack((self.program_counter & 0x00FF) as u8);
    }

    pub fn interrupt_request(&mut self) {
        match self.get_flag(Flags6502::DisableInterrupts) {
            1 => return,
            _ => {
                self.write_counter_to_stack();
                self.set_flag(Flags6502::Break, false);
                self.set_flag(Flags6502::Unused, true);
                self.set_flag(Flags6502::DisableInterrupts, true);
                self.write_to_stack(self.status_register);
                self.program_counter = self.read_program_counter(INTERRUPT_PROGRAM_COUNTER_ADDRESS);
                self.cycles = 7;
            }
        };
    }

    pub fn non_mask_interrupt_request(&mut self) {
        self.write_counter_to_stack();
        self.set_flag(Flags6502::Break, false);
        self.set_flag(Flags6502::Unused, true);
        self.set_flag(Flags6502::DisableInterrupts, true);
        self.write_to_stack(self.status_register);
        self.program_counter = self.read_program_counter(NON_MASK_INTERRUPT_PROGRAM_COUNTER_ADDRESS);
        self.cycles = 8;
    }

    /// Sets or clears a specific bit of the status register
    pub fn set_flag(&mut self, flag: Flags6502, value: bool) {
        if value {
            self.status_register = self.status_register | (flag as u8);
        } else {
            self.status_register = self.status_register & !(flag as u8);
        }
    }

    pub fn get_flag(&mut self, flag: Flags6502) -> u8 {
        if self.status_register & (flag as u8) > 0 { 
            1 
        } else { 
            0 
        }
    }
}

pub enum Flags6502 {
    CarryBit = 1 << 0,
    Zero = 1 << 1,
    DisableInterrupts = 1 << 2,
    DecimalMode = 1 << 3,
    Break = 1 << 4,
    Unused = 1 << 5,
    Overflow = 1 << 6,
    Negative = 1 << 7
}