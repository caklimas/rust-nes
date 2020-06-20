use crate::bus;
use crate::opcode_table;
use crate::address_modes;

const NON_MASK_INTERRUPT_PROGRAM_COUNTER_ADDRESS: u16 = 0xFFFA;

pub const INTERRUPT_PROGRAM_COUNTER_ADDRESS: u16 = 0xFFFE;

pub struct Olc6502 {
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

impl Olc6502 {
    pub fn new() -> Self {
        Olc6502 {
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: 0,
            program_counter: 0,
            status_register: 0x00,
            fetched_data: 0,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0
        }
    }

    pub fn interrupt_request(&mut self, bus: &mut bus::Bus) {
        match self.get_flag(Flags6502::DisableInterrupts) {
            1 => return,
            _ => {
                bus.write_counter_to_stack();
                self.set_flag(Flags6502::Break, false);
                self.set_flag(Flags6502::Unused, true);
                self.set_flag(Flags6502::DisableInterrupts, true);
                bus.write_to_stack(self.status_register);
                self.program_counter = bus.read_program_counter(INTERRUPT_PROGRAM_COUNTER_ADDRESS);
                self.cycles = 7;
            }
        }
    }

    pub fn non_mask_interrupt_request(&mut self, bus: &mut bus::Bus) {
        bus.write_counter_to_stack();
        self.set_flag(Flags6502::Break, false);
        self.set_flag(Flags6502::Unused, true);
        self.set_flag(Flags6502::DisableInterrupts, true);
        bus.write_to_stack(self.status_register);
        self.program_counter = bus.read_program_counter(NON_MASK_INTERRUPT_PROGRAM_COUNTER_ADDRESS);
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

    pub fn is_overflow(&mut self, result: u16) -> bool {
        let data_accum_same_bits =  ((self.fetched_data & 0x80) as u16) ^ ((self.accumulator & 0x80) as u16) != 0x80;
        let data_result_diff_bits = ((self.fetched_data & 0x80) as u16) ^ (result & 0x80) == 0x80;
    
        return data_accum_same_bits && data_result_diff_bits;
    }
}

pub enum Flags6502 {
    CarryBit = (1 << 0),
    Zero = (1 << 1),
    DisableInterrupts = (1 << 2),
    DecimalMode = (1 << 3),
    Break = (1 << 4),
    Unused = (1 << 5),
    Overflow = (1 << 6),
    Negative = (1 << 7)
}