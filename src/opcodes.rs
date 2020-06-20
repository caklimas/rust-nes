use crate::bus;
use crate::cpu;
use crate::opcode_table;
use crate::address_modes;


/// Opcode: Add with Carry
/// Overflow occurs when you add two positive numbers together and get a negative or you add two negative and get a positive
/// To check this you check the most significant bits of accumulator, memory and result
/// In order for overflow to occur, the most significant bits of the accumulator and memory need to be the same and the result needs to be different
/// We can do this using XOR
pub fn adc(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    // Add in u16 space so we can get any carry bits
    let result = (bus.cpu.fetched_data as u16).wrapping_add(bus.cpu.accumulator as u16).wrapping_add(bus.cpu.get_flag(cpu::Flags6502::CarryBit) as u16);
    let overflow = bus.cpu.is_overflow(result);

    bus.cpu.set_flag(cpu::Flags6502::CarryBit, result > 0xFF);
    bus.cpu.set_flag(cpu::Flags6502::Zero, (result & 0x00FF) == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.accumulator & 0x80) != 0);
    bus.cpu.set_flag(cpu::Flags6502::Overflow, overflow);

    bus.cpu.accumulator = (result & 0x00FF) as u8;

    1
}

/// Opcode: Logical AND
pub fn and(bus: &mut bus::Bus) -> u8 {
    bus.fetch();
    bus.cpu.accumulator = bus.cpu.accumulator & bus.cpu.fetched_data;

    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.accumulator == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.accumulator & 0x80) != 0);

    1
}

/// Opcode: Arithmetic Shift Left
pub fn asl(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    let shifted = (bus.cpu.fetched_data as u16) << 1;

    bus.cpu.set_flag(cpu::Flags6502::CarryBit, shifted > 0xFF);
    bus.cpu.set_flag(cpu::Flags6502::Zero, (shifted & 0x00FF) == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (shifted & 0x80) != 0);

    let result = (shifted as u8) & 0xFF;
    match opcode_table::OPCODE_TABLE[bus.cpu.opcode as usize].3 {
        address_modes::AddressMode::Imp => bus.cpu.accumulator = result,
        _ => bus.cpu_write(bus.cpu.addr_abs, result)
    };

    1
}

/// Opcode: Branch if Carry Clear
pub fn bcc(bus: &mut bus::Bus) -> u8 {
    branch_if_clear(bus, cpu::Flags6502::CarryBit);
    0
}

/// Opcode: Branch if Carry Set
pub fn bcs(bus: &mut bus::Bus) -> u8 {
    branch_if_set(bus, cpu::Flags6502::CarryBit);
    0
}

/// Opcode: Branch if Equal
pub fn beq(bus: &mut bus::Bus) -> u8 {
    branch_if_set(bus, cpu::Flags6502::Zero);
    0
}

/// Opcode: Bit Test
pub fn bit(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    let bit6 = (bus.cpu.fetched_data & 0x40) >> 6;
    let bit7 = (bus.cpu.fetched_data & 0x80) >> 7;
    bus.cpu.set_flag(cpu::Flags6502::Zero, (bus.cpu.fetched_data & bus.cpu.accumulator) == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Overflow, bit6 == 1);
    bus.cpu.set_flag(cpu::Flags6502::Negative, bit7 == 1);

    0
}

/// Opcode: Branch if Minus
pub fn bmi(bus: &mut bus::Bus) -> u8 {
    branch_if_set(bus, cpu::Flags6502::Negative);
    0
}

/// Opcode: Branch if Not Equal
pub fn bne(bus: &mut bus::Bus) -> u8 {
    branch_if_clear(bus, cpu::Flags6502::Zero);
    0
}

/// Opcode: Branch if Positive
pub fn bpl(bus: &mut bus::Bus) -> u8 {
    branch_if_clear(bus, cpu::Flags6502::Negative);
    0
}

/// Opcode: Force Interrupt
/// Push program counter and processor status into stack
pub fn brk(bus: &mut bus::Bus) -> u8 {
    bus.cpu.program_counter += 1;
    bus.cpu.set_flag(cpu::Flags6502::Break, true);

    bus.write_counter_to_stack();
    bus.write_to_stack(bus.cpu.status_register);

    bus.cpu.program_counter = bus.read_program_counter(cpu::INTERRUPT_PROGRAM_COUNTER_ADDRESS);
    0
}

/// Opcode: Branch if Overflow Clear
pub fn bvc(bus: &mut bus::Bus) -> u8 {
    branch_if_clear(bus, cpu::Flags6502::Overflow);
    0
}

/// Opcode: Branch if Overflow Set
pub fn bvs(bus: &mut bus::Bus) -> u8 {
    branch_if_set(bus, cpu::Flags6502::Overflow);
    0
}

/// Opcode: Clear Carry Flag
pub fn clc(bus: &mut bus::Bus) -> u8 {
    bus.cpu.set_flag(cpu::Flags6502::CarryBit, false);
    0
}

/// Opcode: Clear Decimal Mode
pub fn cld(bus: &mut bus::Bus) -> u8 {
    bus.cpu.set_flag(cpu::Flags6502::DecimalMode, false);
    0
}

/// Opcode: Clear Interrupt Disable
pub fn cli(bus: &mut bus::Bus) -> u8 {
    bus.cpu.set_flag(cpu::Flags6502::DisableInterrupts, false);
    0
}

/// Opcode: Clear Overflow Flag
pub fn clv(bus: &mut bus::Bus) -> u8 {
    bus.cpu.set_flag(cpu::Flags6502::Overflow, false);
    0
}

/// Opcode: Compare
pub fn cmp(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    let result = (bus.cpu.accumulator as u16).wrapping_sub(bus.cpu.fetched_data as u16);

    bus.cpu.set_flag(cpu::Flags6502::CarryBit, bus.cpu.accumulator >= bus.cpu.fetched_data);
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.accumulator == bus.cpu.fetched_data);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Compare X Register
pub fn cpx(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    let result = (bus.cpu.x_register as u16).wrapping_sub(bus.cpu.fetched_data as u16);

    bus.cpu.set_flag(cpu::Flags6502::CarryBit, bus.cpu.x_register >= bus.cpu.fetched_data);
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.x_register == bus.cpu.fetched_data);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Compare Y Register
pub fn cpy(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    let result = (bus.cpu.y_register as u16).wrapping_sub(bus.cpu.fetched_data as u16);

    bus.cpu.set_flag(cpu::Flags6502::CarryBit, bus.cpu.y_register >= bus.cpu.fetched_data);
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.y_register == bus.cpu.fetched_data);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Decrement Memory
pub fn dec(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    let result = bus.cpu.fetched_data.wrapping_sub(1);
    bus.cpu_write(bus.cpu.addr_abs, result & 0x00FF);

    bus.cpu.set_flag(cpu::Flags6502::Zero, result == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Decrement X Register
pub fn dex(bus: &mut bus::Bus) -> u8 {
    bus.cpu.x_register = bus.cpu.x_register.wrapping_sub(1);

    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.x_register == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.x_register & 0x80) != 0);

    0
}

/// Opcode: Decrement Y Register
pub fn dey(bus: &mut bus::Bus) -> u8 {
    bus.cpu.y_register = bus.cpu.y_register.wrapping_sub(1);

    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.y_register == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.y_register & 0x80) != 0);

    0
}

/// Opcode: Exclusive OR
pub fn eor(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    bus.cpu.accumulator = bus.cpu.accumulator ^ bus.cpu.fetched_data;

    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.accumulator == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.accumulator & 0x80) != 0);

    1
}

/// Opcode: Increment Memory
pub fn inc(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    let result = bus.cpu.fetched_data.wrapping_add(1);
    bus.cpu_write(bus.cpu.addr_abs, result & 0x00FF);

    bus.cpu.set_flag(cpu::Flags6502::Zero, result == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Increment X Register
pub fn inx(bus: &mut bus::Bus) -> u8 {
    bus.cpu.x_register = bus.cpu.x_register.wrapping_add(1);

    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.x_register == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.x_register & 0x80) != 0);

    0
}

/// Opcode: Increment Y Register
pub fn iny(bus: &mut bus::Bus) -> u8 {
    bus.cpu.y_register = bus.cpu.y_register.wrapping_add(1);

    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.y_register == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.y_register & 0x80) != 0);

    0
}

/// Opcode: Jump
pub fn jmp(bus: &mut bus::Bus) -> u8 {
    bus.cpu.program_counter = bus.cpu.addr_abs;
    0
}

/// Opcode: Jump to Subroutine
pub fn jsr(bus: &mut bus::Bus) -> u8 {
    bus.cpu.program_counter -= 1;
    bus.write_counter_to_stack();
    bus.cpu.program_counter = bus.cpu.addr_abs;

    0
}

/// Opcode: Load Accumulator
pub fn lda(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    bus.cpu.accumulator = bus.cpu.fetched_data;
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.accumulator == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.accumulator & 0x80) != 0);

    0
}

/// Opcode: Load X Register
pub fn ldx(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    bus.cpu.x_register = bus.cpu.fetched_data;
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.x_register == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.x_register & 0x80) != 0);

    0
}

/// Opcode: Load Y Register
pub fn ldy(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    bus.cpu.y_register = bus.cpu.fetched_data;
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.y_register == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.y_register & 0x80) != 0);

    0
}

/// Opcode: Logical Shift Right
pub fn lsr(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    let shifted = bus.cpu.fetched_data >> 1;
    bus.cpu.set_flag(cpu::Flags6502::Zero, shifted == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (shifted & 0x80) != 0);
    bus.cpu.set_flag(cpu::Flags6502::CarryBit, (shifted & 0x0001) != 0);

    let result = (shifted as u8) & 0xFF;
    match opcode_table::OPCODE_TABLE[bus.cpu.opcode as usize].3 {
        address_modes::AddressMode::Imp => bus.cpu.accumulator = result,
        _ => bus.cpu_write(bus.cpu.addr_abs, result)
    };

    0
}

/// Opcode: No Operation
pub fn nop(bus: &mut bus::Bus) -> u8 {
    match bus.cpu.opcode {
        0x1C => 1,
        0x3C => 1,
        0x5C => 1,
        0x7C => 1,
        0xDC => 1,
        0xFC => 1,
        _ => 0
    }
}

/// Opcode: Logical Inclusive OR
pub fn ora(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    bus.cpu.accumulator = bus.cpu.accumulator | bus.cpu.fetched_data;
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.accumulator == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.accumulator & 0x80) != 0);

    1
}

/// Opcode: Push Accumulator
pub fn pha(bus: &mut bus::Bus) -> u8 {
    bus.write_to_stack(bus.cpu.accumulator);
    0
}

/// Opcode: Push Processor Status
pub fn php(bus: &mut bus::Bus) -> u8 {
    bus.write_to_stack(bus.cpu.status_register);
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.accumulator == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.accumulator & 0x80) != 0);

    0
}

/// Opcode: Pull Accumulator
pub fn pla(bus: &mut bus::Bus) -> u8 {
    bus.cpu.accumulator = bus.read_from_stack();
    0
}

/// Opcode: Pull Processor Status
pub fn plp(bus: &mut bus::Bus) -> u8 {
    bus.cpu.status_register = bus.read_from_stack();
    set_flags_from_data(bus, bus.cpu.status_register);

    0
}

/// Opcode: Rotate Left
pub fn rol(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    let result = (bus.cpu.fetched_data << 1) | bus.cpu.get_flag(cpu::Flags6502::CarryBit);

    bus.cpu.set_flag(cpu::Flags6502::CarryBit, (result & 0b10000000) != 0);
    bus.cpu.set_flag(cpu::Flags6502::Zero, result == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    match opcode_table::OPCODE_TABLE[bus.cpu.opcode as usize].3 {
        address_modes::AddressMode::Imp => bus.cpu.accumulator = result,
        _ => bus.cpu_write(bus.cpu.addr_abs, result)
    };

    0
}

/// Opcode: Rotate Right
pub fn ror(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    let result = (bus.cpu.get_flag(cpu::Flags6502::CarryBit) << 7) | (bus.cpu.fetched_data >> 1);

    bus.cpu.set_flag(cpu::Flags6502::CarryBit, (result & 0b00000001) != 0);
    bus.cpu.set_flag(cpu::Flags6502::Zero, result == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    match opcode_table::OPCODE_TABLE[bus.cpu.opcode as usize].3 {
        address_modes::AddressMode::Imp => bus.cpu.accumulator = result,
        _ => bus.cpu_write(bus.cpu.addr_abs, result)
    };

    0
}

/// Opcode: Return from Interrupt
pub fn rti(bus: &mut bus::Bus) -> u8 {
    bus.cpu.status_register = bus.read_from_stack();
    bus.cpu.program_counter = bus.read_counter_from_stack();
    set_flags_from_data(bus, bus.cpu.status_register);

    0
}

/// Opcode: Return from Subroutine
pub fn rts(bus: &mut bus::Bus) -> u8 {
    bus.cpu.program_counter = bus.read_counter_from_stack();
    bus.cpu.program_counter += 1;

    0
}

pub fn sbc(bus: &mut bus::Bus) -> u8 {
    bus.fetch();

    // Add in u16 space so we can get any carry bits
    let result = (bus.cpu.fetched_data as u16).wrapping_sub(bus.cpu.accumulator as u16).wrapping_sub((1 - bus.cpu.get_flag(cpu::Flags6502::CarryBit)) as u16);
    let overflow = bus.cpu.is_overflow(result);
    
    bus.cpu.set_flag(cpu::Flags6502::CarryBit, result > 0xFF);
    bus.cpu.set_flag(cpu::Flags6502::Zero, (result & 0x00FF) == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.accumulator & 0x80) != 0);
    bus.cpu.set_flag(cpu::Flags6502::Overflow, overflow);

    bus.cpu.accumulator = (result & 0x00FF) as u8;

    1
}

/// Opcode: Set Carry Flag
pub fn sec(bus: &mut bus::Bus) -> u8 {
    bus.cpu.set_flag(cpu::Flags6502::CarryBit, true);
    0
}

/// Opcode: Set Decimal Flag
pub fn sed(bus: &mut bus::Bus) -> u8 {
    bus.cpu.set_flag(cpu::Flags6502::DecimalMode, true);
    0
}

/// Opcode: Set Interrupt Disable
pub fn sei(bus: &mut bus::Bus) -> u8 {
    bus.cpu.set_flag(cpu::Flags6502::DisableInterrupts, true);
    0
}

/// Opcode: Store Accumulator
pub fn sta(bus: &mut bus::Bus) -> u8 {
    bus.cpu_write(bus.cpu.addr_abs, bus.cpu.accumulator);
    0
}

/// Opcode: Store X Register
pub fn stx(bus: &mut bus::Bus) -> u8 {
    bus.cpu_write(bus.cpu.addr_abs, bus.cpu.x_register);
    0
}

/// Opcode: Store Y Register
pub fn sty(bus: &mut bus::Bus) -> u8 {
    bus.cpu_write(bus.cpu.addr_abs, bus.cpu.y_register);
    0
}

/// Opcode: Transfer Accumulator to X
pub fn tax(bus: &mut bus::Bus) -> u8 {
    bus.cpu.x_register = bus.cpu.accumulator;
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.x_register == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.x_register & 0x80) != 0);

    0
}

/// Opcode: Transfer Accumulator to Y
pub fn tay(bus: &mut bus::Bus) -> u8 {
    bus.cpu.y_register = bus.cpu.accumulator;
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.y_register == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.y_register & 0x80) != 0);

    0
}

/// Opcode: Transfer Stack Pointer to X
pub fn tsx(bus: &mut bus::Bus) -> u8 {
    bus.cpu.x_register = bus.cpu.stack_pointer;
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.x_register == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.x_register & 0x80) != 0);

    0
}

/// Opcode: Transfer X to Accumulator
pub fn txa(bus: &mut bus::Bus) -> u8 {
    bus.cpu.accumulator = bus.cpu.x_register;
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.accumulator == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.accumulator & 0x80) != 0);

    0
}

/// Opcode: Transfer X to Stack Pointer
pub fn txs(bus: &mut bus::Bus) -> u8 {
    bus.cpu.stack_pointer = bus.cpu.x_register;
    0
}

/// Opcode: Transfer Y to Accumulator
pub fn tya(bus: &mut bus::Bus) -> u8 {
    bus.cpu.accumulator = bus.cpu.y_register;
    bus.cpu.set_flag(cpu::Flags6502::Zero, bus.cpu.accumulator == 0x00);
    bus.cpu.set_flag(cpu::Flags6502::Negative, (bus.cpu.accumulator & 0x80) != 0);

    0
}

/// This is used when it is an illegal opcode and does nothing
pub fn xxx(_bus: &mut bus::Bus) -> u8 {
    0
}

fn branch_if_set(bus: &mut bus::Bus, flag: cpu::Flags6502) {
    branch_if_flag_equal(bus, flag, 1);
}

fn branch_if_clear(bus: &mut bus::Bus, flag: cpu::Flags6502) {
    branch_if_flag_equal(bus, flag, 0);
}

fn branch_if_flag_equal(bus: &mut bus::Bus, flag: cpu::Flags6502, value: u8) {
    if bus.cpu.get_flag(flag) != value {
        return;
    }

    bus.cpu.addr_abs = bus.cpu.program_counter.wrapping_add(bus.cpu.addr_rel);
    bus.cpu.cycles += 1;

    // If the addition caused paging, add another cycle
    if (bus.cpu.addr_abs & 0xFF00) != (bus.cpu.program_counter & 0xFF00) {
        bus.cpu.cycles += 1;
    }

    bus.cpu.program_counter = bus.cpu.addr_abs;
}

fn set_flags_from_data(bus: &mut bus::Bus, data: u8) {
    bus.cpu.set_flag(cpu::Flags6502::CarryBit, data          & 0b00000001 > 0);
    bus.cpu.set_flag(cpu::Flags6502::Zero, data              & 0b00000010 > 0);
    bus.cpu.set_flag(cpu::Flags6502::DisableInterrupts, data & 0b00000100 > 0);
    bus.cpu.set_flag(cpu::Flags6502::DecimalMode, data       & 0b00001000 > 0);
    bus.cpu.set_flag(cpu::Flags6502::Overflow, data          & 0b01000000 > 0);
    bus.cpu.set_flag(cpu::Flags6502::Negative, data          & 0b10000000 > 0);
}

#[cfg(test)]
mod tests {
    use crate::cpu;
    use crate::opcodes::*;

    #[test]
    fn adc_sets_carry_flag_correctly() {
        let address = 0;
        let mut bus = bus::Bus::new();
        bus.cpu.opcode = 0x79;
        bus.cpu.addr_abs = address;
        bus.cpu_ram[address as usize] = 255;
        bus.cpu.accumulator = 1;

        adc(&mut bus);

        assert_eq!(bus.cpu.get_flag(cpu::Flags6502::CarryBit), 1);
    }

    #[test]
    fn adc_sets_zero_flag_correctly() {
        let address = 0;
        let mut bus = bus::Bus::new();
        bus.cpu.opcode = 0x79;
        bus.cpu.addr_abs = address;
        bus.cpu_ram[address as usize] = 0;
        bus.cpu.accumulator = 0;

        adc(&mut bus);

        assert_eq!(bus.cpu.get_flag(cpu::Flags6502::Zero), 1);
    }

    #[test]
    fn adc_sets_negative_flag_correctly() {
        let address = 0;
        let mut bus = bus::Bus::new();
        bus.cpu.opcode = 0x79;
        bus.cpu.addr_abs = address;
        bus.cpu_ram[address as usize] = 0;
        bus.cpu.accumulator = 0b10000001;

        adc(&mut bus);

        assert_eq!(bus.cpu.get_flag(cpu::Flags6502::Negative), 1);
    }

    #[test]
    fn adc_sets_overflow_flag_correctly() {
        let address = 0;
        let mut bus = bus::Bus::new();
        bus.cpu.opcode = 0x79;
        bus.cpu.addr_abs = address;
        bus.cpu_ram[address as usize] = 127;
        bus.cpu.accumulator = 4;

        adc(&mut bus);

        assert_eq!(bus.cpu.get_flag(cpu::Flags6502::Overflow), 1);
    }

    #[test]
    fn adc_adds_correctly() {
        let address = 0;
        let mut bus = bus::Bus::new();
        bus.cpu.opcode = 0x79;
        bus.cpu.addr_abs = address;
        bus.cpu_ram[address as usize] = 200;
        bus.cpu.accumulator = 4;

        adc(&mut bus);

        assert_eq!(bus.cpu.accumulator, 204);
    }

    #[test]
    fn sbc_subtracts_correctly() {
        let address = 0;
        let mut bus = bus::Bus::new();
        bus.cpu.set_flag(cpu::Flags6502::CarryBit, true);
        bus.cpu.opcode = 0xF1;
        bus.cpu.addr_abs = address;
        bus.cpu_ram[address as usize] = 5;
        bus.cpu.accumulator = 3;

        sbc(&mut bus);

        assert_eq!(bus.cpu.accumulator, 2);
    }

    #[test]
    fn clc_sets_carry_flag_correctly() {
        let mut bus = bus::Bus::new();
        bus.cpu.set_flag(cpu::Flags6502::CarryBit, true);
        clc(&mut bus);

        assert_eq!(bus.cpu.get_flag(cpu::Flags6502::CarryBit), 0);
    }
}