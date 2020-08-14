use super::{Cpu6502, Flags6502, INTERRUPT_PROGRAM_COUNTER_ADDRESS};
use super::opcode_table;
use super::address_modes;

/// Opcode: Add with Carry
/// Overflow occurs when you add two positive numbers together and get a negative or you add two negative and get a positive
/// To check this you check the most significant bits of accumulator, memory and result
/// In order for overflow to occur, the most significant bits of the accumulator and memory need to be the same and the result needs to be different
/// We can do this using XOR
pub fn adc(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    // Add in u16 space so we can get any carry bits
    let (result1, overflow1) = cpu.fetched_data.overflowing_add(cpu.accumulator);
    let (result, overflow2) = result1.overflowing_add(cpu.get_flag(Flags6502::CarryBit));
    let overflow = (cpu.accumulator ^ cpu.fetched_data) & 0x80 == 0 && (cpu.accumulator ^ result) & 0x80 != 0;

    cpu.set_flag(Flags6502::CarryBit, overflow1 | overflow2);
    cpu.set_flag(Flags6502::Zero, result == 0x00);
    cpu.set_flag(Flags6502::Negative, (result & 0x80) != 0);
    cpu.set_flag(Flags6502::Overflow, overflow);

    cpu.accumulator = result;

    1
}

/// Opcode: Logical AND
pub fn and(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();
    cpu.accumulator &= cpu.fetched_data;

    cpu.set_flag(Flags6502::Zero, cpu.accumulator == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.accumulator & 0x80) != 0);

    1
}

/// Opcode: Arithmetic Shift Left
pub fn asl(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    let shifted = (cpu.fetched_data as u16) << 1;

    cpu.set_flag(Flags6502::CarryBit, shifted > 0xFF);
    cpu.set_flag(Flags6502::Zero, shifted.trailing_zeros() >= 8);
    cpu.set_flag(Flags6502::Negative, (shifted & 0x80) != 0);

    let result = shifted as u8;
    match opcode_table::OPCODE_TABLE[cpu.opcode as usize].3 {
        address_modes::AddressMode::Imp => cpu.accumulator = result,
        _ => cpu.write(cpu.addr_abs, result)
    };

    1
}

/// Opcode: Branch if Carry Clear
pub fn bcc(cpu: &mut Cpu6502) -> u8 {
    branch_if_clear(cpu, Flags6502::CarryBit);
    0
}

/// Opcode: Branch if Carry Set
pub fn bcs(cpu: &mut Cpu6502) -> u8 {
    branch_if_set(cpu, Flags6502::CarryBit);
    0
}

/// Opcode: Branch if Equal
pub fn beq(cpu: &mut Cpu6502) -> u8 {
    branch_if_set(cpu, Flags6502::Zero);
    0
}

/// Opcode: Bit Test
pub fn bit(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    let bit6 = (cpu.fetched_data & 0x40) >> 6;
    let bit7 = (cpu.fetched_data & 0x80) >> 7;
    cpu.set_flag(Flags6502::Zero, (cpu.fetched_data & cpu.accumulator) == 0x00);
    cpu.set_flag(Flags6502::Overflow, bit6 == 1);
    cpu.set_flag(Flags6502::Negative, bit7 == 1);

    0
}

/// Opcode: Branch if Minus
pub fn bmi(cpu: &mut Cpu6502) -> u8 {
    branch_if_set(cpu, Flags6502::Negative);
    0
}

/// Opcode: Branch if Not Equal
pub fn bne(cpu: &mut Cpu6502) -> u8 {
    branch_if_clear(cpu, Flags6502::Zero);
    0
}

/// Opcode: Branch if Positive
pub fn bpl(cpu: &mut Cpu6502) -> u8 {
    branch_if_clear(cpu, Flags6502::Negative);
    0
}

/// Opcode: Force Interrupt
/// Push program counter and processor status into stack
pub fn brk(cpu: &mut Cpu6502) -> u8 {
    cpu.write_counter_to_stack();
    cpu.write_to_stack(cpu.status_register | 0b00110000);

    cpu.set_flag(Flags6502::DisableInterrupts, true);
    cpu.program_counter = cpu.read_program_counter(INTERRUPT_PROGRAM_COUNTER_ADDRESS);

    0
}

/// Opcode: Branch if Overflow Clear
pub fn bvc(cpu: &mut Cpu6502) -> u8 {
    branch_if_clear(cpu, Flags6502::Overflow);
    0
}

/// Opcode: Branch if Overflow Set
pub fn bvs(cpu: &mut Cpu6502) -> u8 {
    branch_if_set(cpu, Flags6502::Overflow);
    0
}

/// Opcode: Clear Carry Flag
pub fn clc(cpu: &mut Cpu6502) -> u8 {
    cpu.set_flag(Flags6502::CarryBit, false);
    0
}

/// Opcode: Clear Decimal Mode
pub fn cld(cpu: &mut Cpu6502) -> u8 {
    cpu.set_flag(Flags6502::DecimalMode, false);
    0
}

/// Opcode: Clear Interrupt Disable
pub fn cli(cpu: &mut Cpu6502) -> u8 {
    cpu.set_flag(Flags6502::DisableInterrupts, false);
    0
}

/// Opcode: Clear Overflow Flag
pub fn clv(cpu: &mut Cpu6502) -> u8 {
    cpu.set_flag(Flags6502::Overflow, false);
    0
}

/// Opcode: Compare
pub fn cmp(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    let result = (cpu.accumulator as u16).wrapping_sub(cpu.fetched_data as u16);

    cpu.set_flag(Flags6502::CarryBit, cpu.accumulator >= cpu.fetched_data);
    cpu.set_flag(Flags6502::Zero, cpu.accumulator == cpu.fetched_data);
    cpu.set_flag(Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Compare X Register
pub fn cpx(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    let result = (cpu.x_register as u16).wrapping_sub(cpu.fetched_data as u16);

    cpu.set_flag(Flags6502::CarryBit, cpu.x_register >= cpu.fetched_data);
    cpu.set_flag(Flags6502::Zero, cpu.x_register == cpu.fetched_data);
    cpu.set_flag(Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Compare Y Register
pub fn cpy(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    let result = (cpu.y_register as u16).wrapping_sub(cpu.fetched_data as u16);

    cpu.set_flag(Flags6502::CarryBit, cpu.y_register >= cpu.fetched_data);
    cpu.set_flag(Flags6502::Zero, cpu.y_register == cpu.fetched_data);
    cpu.set_flag(Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Decrement Memory
pub fn dec(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    let result = cpu.fetched_data.wrapping_sub(1);
    cpu.write(cpu.addr_abs, result);

    cpu.set_flag(Flags6502::Zero, result == 0x00);
    cpu.set_flag(Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Decrement X Register
pub fn dex(cpu: &mut Cpu6502) -> u8 {
    cpu.x_register = cpu.x_register.wrapping_sub(1);

    cpu.set_flag(Flags6502::Zero, cpu.x_register == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.x_register & 0x80) != 0);

    0
}

/// Opcode: Decrement Y Register
pub fn dey(cpu: &mut Cpu6502) -> u8 {
    cpu.y_register = cpu.y_register.wrapping_sub(1);

    cpu.set_flag(Flags6502::Zero, cpu.y_register == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.y_register & 0x80) != 0);

    0
}

/// Opcode: Exclusive OR
pub fn eor(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    cpu.accumulator ^= cpu.fetched_data;

    cpu.set_flag(Flags6502::Zero, cpu.accumulator == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.accumulator & 0x80) != 0);

    1
}

/// Opcode: Increment Memory
pub fn inc(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    let result = cpu.fetched_data.wrapping_add(1);
    cpu.write(cpu.addr_abs, result);

    cpu.set_flag(Flags6502::Zero, result == 0x00);
    cpu.set_flag(Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Increment X Register
pub fn inx(cpu: &mut Cpu6502) -> u8 {
    cpu.x_register = cpu.x_register.wrapping_add(1);

    cpu.set_flag(Flags6502::Zero, cpu.x_register == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.x_register & 0x80) != 0);

    0
}

/// Opcode: Increment Y Register
pub fn iny(cpu: &mut Cpu6502) -> u8 {
    cpu.y_register = cpu.y_register.wrapping_add(1);

    cpu.set_flag(Flags6502::Zero, cpu.y_register == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.y_register & 0x80) != 0);

    0
}

/// Opcode: Jump
pub fn jmp(cpu: &mut Cpu6502) -> u8 {
    cpu.program_counter = cpu.addr_abs;
    0
}

/// Opcode: Jump to Subroutine
pub fn jsr(cpu: &mut Cpu6502) -> u8 {
    cpu.program_counter -= 1;
    cpu.write_counter_to_stack();
    cpu.program_counter = cpu.addr_abs;

    0
}

/// Opcode: Load Accumulator
pub fn lda(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    cpu.accumulator = cpu.fetched_data;
    cpu.set_flag(Flags6502::Zero, cpu.accumulator == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.accumulator & 0x80) != 0);

    0
}

/// Opcode: Load X Register
pub fn ldx(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    cpu.x_register = cpu.fetched_data;
    cpu.set_flag(Flags6502::Zero, cpu.x_register == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.x_register & 0x80) != 0);

    0
}

/// Opcode: Load Y Register
pub fn ldy(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    cpu.y_register = cpu.fetched_data;
    cpu.set_flag(Flags6502::Zero, cpu.y_register == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.y_register & 0x80) != 0);

    0
}

/// Opcode: Logical Shift Right
pub fn lsr(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    let shifted = cpu.fetched_data >> 1;
    cpu.set_flag(Flags6502::Zero, shifted == 0x00);
    cpu.set_flag(Flags6502::Negative, (shifted & 0x80) != 0);
    cpu.set_flag(Flags6502::CarryBit, (cpu.fetched_data & 0x0001) != 0);
    
    let result = shifted as u8;
    match opcode_table::OPCODE_TABLE[cpu.opcode as usize].3 {
        address_modes::AddressMode::Imp => cpu.accumulator = result,
        _ => cpu.write(cpu.addr_abs, result)
    };

    0
}

/// Opcode: No Operation
pub fn nop(cpu: &mut Cpu6502) -> u8 {
    match cpu.opcode {
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
pub fn ora(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    cpu.accumulator |= cpu.fetched_data;
    cpu.set_flag(Flags6502::Zero, cpu.accumulator == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.accumulator & 0x80) != 0);

    1
}

/// Opcode: Push Accumulator
pub fn pha(cpu: &mut Cpu6502) -> u8 {
    cpu.write_to_stack(cpu.accumulator);
    0
}

/// Opcode: Push Processor Status
pub fn php(cpu: &mut Cpu6502) -> u8 {
    cpu.write_to_stack(cpu.status_register | (Flags6502::Break as u8) | (Flags6502::Unused as u8));
    0
}

/// Opcode: Pull Accumulator
pub fn pla(cpu: &mut Cpu6502) -> u8 {
    cpu.accumulator = cpu.read_from_stack();
    cpu.set_flag(Flags6502::Zero, cpu.accumulator == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.accumulator & 0x80) != 0);
    0
}

/// Opcode: Pull Processor Status
pub fn plp(cpu: &mut Cpu6502) -> u8 {
    cpu.status_register = cpu.read_from_stack();
    0
}

/// Opcode: Rotate Left
pub fn rol(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    let result = (cpu.fetched_data << 1) | cpu.get_flag(Flags6502::CarryBit);

    cpu.set_flag(Flags6502::CarryBit, (cpu.fetched_data & 0b10000000) != 0);
    cpu.set_flag(Flags6502::Zero, result == 0x00);
    cpu.set_flag(Flags6502::Negative, (result & 0x80) != 0);

    match opcode_table::OPCODE_TABLE[cpu.opcode as usize].3 {
        address_modes::AddressMode::Imp => cpu.accumulator = result,
        _ => cpu.write(cpu.addr_abs, result)
    };

    0
}

/// Opcode: Rotate Right
pub fn ror(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    let result = (cpu.get_flag(Flags6502::CarryBit) << 7) | (cpu.fetched_data >> 1);

    cpu.set_flag(Flags6502::CarryBit, (cpu.fetched_data & 0b00000001) != 0);
    cpu.set_flag(Flags6502::Zero, result == 0x00);
    cpu.set_flag(Flags6502::Negative, (result & 0x80) != 0);

    match opcode_table::OPCODE_TABLE[cpu.opcode as usize].3 {
        address_modes::AddressMode::Imp => cpu.accumulator = result,
        _ => cpu.write(cpu.addr_abs, result)
    };

    0
}

/// Opcode: Return from Interrupt
pub fn rti(cpu: &mut Cpu6502) -> u8 {
    plp(cpu);
    cpu.program_counter = cpu.read_counter_from_stack();
    0
}

/// Opcode: Return from Subroutine
pub fn rts(cpu: &mut Cpu6502) -> u8 {
    cpu.program_counter = cpu.read_counter_from_stack();
    cpu.program_counter = cpu.program_counter.wrapping_add(1);

    0
}

pub fn sbc(cpu: &mut Cpu6502) -> u8 {
    cpu.fetch();

    // Add in u16 space so we can get any carry bits
    let (result1, overflow1) = cpu.accumulator.overflowing_sub(cpu.fetched_data);
    let (result, overflow2) = result1.overflowing_sub(1 - cpu.get_flag(Flags6502::CarryBit));
    let overflow = (cpu.accumulator ^ cpu.fetched_data) & 0x80 != 0 && (cpu.accumulator ^ result) & 0x80 != 0;

    cpu.set_flag(Flags6502::CarryBit, !(overflow1 | overflow2));
    cpu.set_flag(Flags6502::Zero, result.trailing_zeros() >= 8);
    cpu.set_flag(Flags6502::Negative, result > 0x7F);
    cpu.set_flag(Flags6502::Overflow, overflow);

    cpu.accumulator = result;

    1
}

/// Opcode: Set Carry Flag
pub fn sec(cpu: &mut Cpu6502) -> u8 {
    cpu.set_flag(Flags6502::CarryBit, true);
    0
}

/// Opcode: Set Decimal Flag
pub fn sed(cpu: &mut Cpu6502) -> u8 {
    cpu.set_flag(Flags6502::DecimalMode, true);
    0
}

/// Opcode: Set Interrupt Disable
pub fn sei(cpu: &mut Cpu6502) -> u8 {
    cpu.set_flag(Flags6502::DisableInterrupts, true);
    0
}

/// Opcode: Store Accumulator
pub fn sta(cpu: &mut Cpu6502) -> u8 {
    cpu.write(cpu.addr_abs, cpu.accumulator);
    0
}

/// Opcode: Store X Register
pub fn stx(cpu: &mut Cpu6502) -> u8 {
    cpu.write(cpu.addr_abs, cpu.x_register);
    0
}

/// Opcode: Store Y Register
pub fn sty(cpu: &mut Cpu6502) -> u8 {
    cpu.write(cpu.addr_abs, cpu.y_register);
    0
}

/// Opcode: Transfer Accumulator to X
pub fn tax(cpu: &mut Cpu6502) -> u8 {
    cpu.x_register = cpu.accumulator;
    cpu.set_flag(Flags6502::Zero, cpu.x_register == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.x_register & 0x80) != 0);

    0
}

/// Opcode: Transfer Accumulator to Y
pub fn tay(cpu: &mut Cpu6502) -> u8 {
    cpu.y_register = cpu.accumulator;
    cpu.set_flag(Flags6502::Zero, cpu.y_register == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.y_register & 0x80) != 0);

    0
}

/// Opcode: Transfer Stack Pointer to X
pub fn tsx(cpu: &mut Cpu6502) -> u8 {
    cpu.x_register = cpu.stack_pointer;
    cpu.set_flag(Flags6502::Zero, cpu.x_register == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.x_register & 0x80) != 0);

    0
}

/// Opcode: Transfer X to Accumulator
pub fn txa(cpu: &mut Cpu6502) -> u8 {
    cpu.accumulator = cpu.x_register;
    cpu.set_flag(Flags6502::Zero, cpu.accumulator == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.accumulator & 0x80) != 0);

    0
}

/// Opcode: Transfer X to Stack Pointer
pub fn txs(cpu: &mut Cpu6502) -> u8 {
    cpu.stack_pointer = cpu.x_register;
    0
}

/// Opcode: Transfer Y to Accumulator
pub fn tya(cpu: &mut Cpu6502) -> u8 {
    cpu.accumulator = cpu.y_register;
    cpu.set_flag(Flags6502::Zero, cpu.accumulator == 0x00);
    cpu.set_flag(Flags6502::Negative, (cpu.accumulator & 0x80) != 0);

    0
}

fn branch_if_set(cpu: &mut Cpu6502, flag: Flags6502) {
    branch_if_flag_equal(cpu, flag, 1);
}

fn branch_if_clear(cpu: &mut Cpu6502, flag: Flags6502) {
    branch_if_flag_equal(cpu, flag, 0);
}

fn branch_if_flag_equal(cpu: &mut Cpu6502, flag: Flags6502, value: u8) {
    if cpu.get_flag(flag) != value {
        return;
    }

    cpu.addr_abs = cpu.program_counter.wrapping_add(cpu.addr_rel);
    cpu.cycles += 1;

    // If the addition caused paging, add another cycle
    if (cpu.addr_abs & 0xFF00) != (cpu.program_counter & 0xFF00) {
        cpu.cycles += 1;
    }

    cpu.program_counter = cpu.addr_abs;
}