use crate::bus;
use crate::opcode_table;
use crate::address_modes;

pub struct olc6502 {
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

impl olc6502 {
    pub fn new() -> Self {
        olc6502 {
            bus: bus::Bus::new(),
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: 0,
            program_counter: 0,
            status_register: 0,
            fetched_data: 0,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0
        }
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.bus.read(self.program_counter, false);
            self.program_counter += 1;

            let record = &opcode_table::OPCODE_TABLE[self.opcode as usize];
            self.cycles = record.4;
            
            let additional_cycle_1 = record.2(self);
            let additional_cycle_2 = record.1(self);

            self.cycles += additional_cycle_1 & additional_cycle_2;
        }

        self.cycles -= 1;
    }

    pub fn reset(&mut self) {
    }

    pub fn interrupt_request(&mut self) {

    }

    pub fn non_mask_interrupt_request(&mut self) {

    }

    pub fn read(&mut self, address: u16, readonly: bool) -> u8 {
        self.bus.read(address, readonly)
    }

    fn fetch(&mut self) -> u8 {
        self.fetched_data = match opcode_table::OPCODE_TABLE[self.opcode as usize].3 {
            address_modes::AddressMode::Imp => self.fetched_data,
            _ => self.read(self.addr_abs, false)
        };
        
        self.fetched_data
    }
}

enum Flags6502 {
    CarryBit = (1 << 0),
    Zero = (1 << 1),
    DisableInterrupts = (1 << 2),
    DecimalMode = (1 << 3),
    Break = (1 << 4),
    Unused = (1 << 5),
    Overflow = (1 << 6),
    Negative = (1 << 7)
}