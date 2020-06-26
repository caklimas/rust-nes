use crate::cpu;
use crate::opcode_table;
use crate::address_modes;


/// Opcode: Add with Carry
/// Overflow occurs when you add two positive numbers together and get a negative or you add two negative and get a positive
/// To check this you check the most significant bits of accumulator, memory and result
/// In order for overflow to occur, the most significant bits of the accumulator and memory need to be the same and the result needs to be different
/// We can do this using XOR
pub fn adc(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    // Add in u16 space so we can get any carry bits
    let (result1, overflow1) = olc.fetched_data.overflowing_add(olc.accumulator);
    let (result, overflow2) = result1.overflowing_add(olc.get_flag(cpu::Flags6502::CarryBit));
    let overflow = (olc.accumulator ^ olc.fetched_data) & 0x80 == 0 && (olc.accumulator ^ result) & 0x80 != 0;

    olc.set_flag(cpu::Flags6502::CarryBit, overflow1 | overflow2);
    olc.set_flag(cpu::Flags6502::Zero, (result & 0x00FF) == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);
    olc.set_flag(cpu::Flags6502::Overflow, overflow);

    olc.accumulator = (result & 0x00FF) as u8;

    1
}

/// Opcode: Logical AND
pub fn and(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();
    olc.accumulator = olc.accumulator & olc.fetched_data;

    olc.set_flag(cpu::Flags6502::Zero, olc.accumulator == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.accumulator & 0x80) != 0);

    1
}

/// Opcode: Arithmetic Shift Left
pub fn asl(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    let shifted = (olc.fetched_data as u16) << 1;

    olc.set_flag(cpu::Flags6502::CarryBit, shifted > 0xFF);
    olc.set_flag(cpu::Flags6502::Zero, (shifted & 0x00FF) == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (shifted & 0x80) != 0);

    let result = (shifted as u8) & 0xFF;
    match opcode_table::OPCODE_TABLE[olc.opcode as usize].3 {
        address_modes::AddressMode::Imp => olc.accumulator = result,
        _ => olc.write(olc.addr_abs, result)
    };

    1
}

/// Opcode: Branch if Carry Clear
pub fn bcc(olc: &mut cpu::Olc6502) -> u8 {
    branch_if_clear(olc, cpu::Flags6502::CarryBit);
    0
}

/// Opcode: Branch if Carry Set
pub fn bcs(olc: &mut cpu::Olc6502) -> u8 {
    branch_if_set(olc, cpu::Flags6502::CarryBit);
    0
}

/// Opcode: Branch if Equal
pub fn beq(olc: &mut cpu::Olc6502) -> u8 {
    branch_if_set(olc, cpu::Flags6502::Zero);
    0
}

/// Opcode: Bit Test
pub fn bit(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    let bit6 = (olc.fetched_data & 0x40) >> 6;
    let bit7 = (olc.fetched_data & 0x80) >> 7;
    olc.set_flag(cpu::Flags6502::Zero, (olc.fetched_data & olc.accumulator) == 0x00);
    olc.set_flag(cpu::Flags6502::Overflow, bit6 == 1);
    olc.set_flag(cpu::Flags6502::Negative, bit7 == 1);

    0
}

/// Opcode: Branch if Minus
pub fn bmi(olc: &mut cpu::Olc6502) -> u8 {
    branch_if_set(olc, cpu::Flags6502::Negative);
    0
}

/// Opcode: Branch if Not Equal
pub fn bne(olc: &mut cpu::Olc6502) -> u8 {
    branch_if_clear(olc, cpu::Flags6502::Zero);
    0
}

/// Opcode: Branch if Positive
pub fn bpl(olc: &mut cpu::Olc6502) -> u8 {
    branch_if_clear(olc, cpu::Flags6502::Negative);
    0
}

/// Opcode: Force Interrupt
/// Push program counter and processor status into stack
pub fn brk(olc: &mut cpu::Olc6502) -> u8 {
    olc.program_counter += 1;
    olc.set_flag(cpu::Flags6502::Break, true);
    olc.set_flag(cpu::Flags6502::DisableInterrupts, true);

    olc.write_counter_to_stack();
    olc.write_to_stack(olc.status_register);
    olc.program_counter = olc.read_program_counter(cpu::INTERRUPT_PROGRAM_COUNTER_ADDRESS);

    0
}

/// Opcode: Branch if Overflow Clear
pub fn bvc(olc: &mut cpu::Olc6502) -> u8 {
    branch_if_clear(olc, cpu::Flags6502::Overflow);
    0
}

/// Opcode: Branch if Overflow Set
pub fn bvs(olc: &mut cpu::Olc6502) -> u8 {
    branch_if_set(olc, cpu::Flags6502::Overflow);
    0
}

/// Opcode: Clear Carry Flag
pub fn clc(olc: &mut cpu::Olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::CarryBit, false);
    0
}

/// Opcode: Clear Decimal Mode
pub fn cld(olc: &mut cpu::Olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::DecimalMode, false);
    0
}

/// Opcode: Clear Interrupt Disable
pub fn cli(olc: &mut cpu::Olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::DisableInterrupts, false);
    0
}

/// Opcode: Clear Overflow Flag
pub fn clv(olc: &mut cpu::Olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::Overflow, false);
    0
}

/// Opcode: Compare
pub fn cmp(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    let result = (olc.accumulator as u16).wrapping_sub(olc.fetched_data as u16);

    olc.set_flag(cpu::Flags6502::CarryBit, olc.accumulator >= olc.fetched_data);
    olc.set_flag(cpu::Flags6502::Zero, olc.accumulator == olc.fetched_data);
    olc.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Compare X Register
pub fn cpx(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    let result = (olc.x_register as u16).wrapping_sub(olc.fetched_data as u16);

    olc.set_flag(cpu::Flags6502::CarryBit, olc.x_register >= olc.fetched_data);
    olc.set_flag(cpu::Flags6502::Zero, olc.x_register == olc.fetched_data);
    olc.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Compare Y Register
pub fn cpy(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    let result = (olc.y_register as u16).wrapping_sub(olc.fetched_data as u16);

    olc.set_flag(cpu::Flags6502::CarryBit, olc.y_register >= olc.fetched_data);
    olc.set_flag(cpu::Flags6502::Zero, olc.y_register == olc.fetched_data);
    olc.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Decrement Memory
pub fn dec(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    let result = olc.fetched_data.wrapping_sub(1);
    olc.write(olc.addr_abs, result & 0x00FF);

    olc.set_flag(cpu::Flags6502::Zero, result == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Decrement X Register
pub fn dex(olc: &mut cpu::Olc6502) -> u8 {
    olc.x_register = olc.x_register.wrapping_sub(1);

    olc.set_flag(cpu::Flags6502::Zero, olc.x_register == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.x_register & 0x80) != 0);

    0
}

/// Opcode: Decrement Y Register
pub fn dey(olc: &mut cpu::Olc6502) -> u8 {
    olc.y_register = olc.y_register.wrapping_sub(1);

    olc.set_flag(cpu::Flags6502::Zero, olc.y_register == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.y_register & 0x80) != 0);

    0
}

/// Opcode: Exclusive OR
pub fn eor(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    olc.accumulator = olc.accumulator ^ olc.fetched_data;

    olc.set_flag(cpu::Flags6502::Zero, olc.accumulator == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.accumulator & 0x80) != 0);

    1
}

/// Opcode: Increment Memory
pub fn inc(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    let result = olc.fetched_data.wrapping_add(1);
    olc.write(olc.addr_abs, result & 0x00FF);

    olc.set_flag(cpu::Flags6502::Zero, result == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Increment X Register
pub fn inx(olc: &mut cpu::Olc6502) -> u8 {
    olc.x_register = olc.x_register.wrapping_add(1);

    olc.set_flag(cpu::Flags6502::Zero, olc.x_register == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.x_register & 0x80) != 0);

    0
}

/// Opcode: Increment Y Register
pub fn iny(olc: &mut cpu::Olc6502) -> u8 {
    olc.y_register = olc.y_register.wrapping_add(1);

    olc.set_flag(cpu::Flags6502::Zero, olc.y_register == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.y_register & 0x80) != 0);

    0
}

/// Opcode: Jump
pub fn jmp(olc: &mut cpu::Olc6502) -> u8 {
    olc.program_counter = olc.addr_abs;
    0
}

/// Opcode: Jump to Subroutine
pub fn jsr(olc: &mut cpu::Olc6502) -> u8 {
    olc.program_counter -= 1;
    olc.write_counter_to_stack();
    olc.program_counter = olc.addr_abs;

    0
}

/// Opcode: Load Accumulator
pub fn lda(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    olc.accumulator = olc.fetched_data;
    olc.set_flag(cpu::Flags6502::Zero, olc.accumulator == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.accumulator & 0x80) != 0);

    0
}

/// Opcode: Load X Register
pub fn ldx(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    olc.x_register = olc.fetched_data;
    olc.set_flag(cpu::Flags6502::Zero, olc.x_register == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.x_register & 0x80) != 0);

    0
}

/// Opcode: Load Y Register
pub fn ldy(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    olc.y_register = olc.fetched_data;
    olc.set_flag(cpu::Flags6502::Zero, olc.y_register == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.y_register & 0x80) != 0);

    0
}

/// Opcode: Logical Shift Right
pub fn lsr(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    let shifted = olc.fetched_data >> 1;
    olc.set_flag(cpu::Flags6502::Zero, shifted == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (shifted & 0x80) != 0);
    olc.set_flag(cpu::Flags6502::CarryBit, (olc.fetched_data & 0x0001) != 0);
    
    let result = (shifted as u8) & 0xFF;
    match opcode_table::OPCODE_TABLE[olc.opcode as usize].3 {
        address_modes::AddressMode::Imp => olc.accumulator = result,
        _ => olc.write(olc.addr_abs, result)
    };

    0
}

/// Opcode: No Operation
pub fn nop(olc: &mut cpu::Olc6502) -> u8 {
    match olc.opcode {
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
pub fn ora(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    olc.accumulator = olc.accumulator | olc.fetched_data;
    olc.set_flag(cpu::Flags6502::Zero, olc.accumulator == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.accumulator & 0x80) != 0);

    1
}

/// Opcode: Push Accumulator
pub fn pha(olc: &mut cpu::Olc6502) -> u8 {
    olc.write_to_stack(olc.accumulator);
    0
}

/// Opcode: Push Processor Status
pub fn php(olc: &mut cpu::Olc6502) -> u8 {
    olc.write_to_stack(olc.status_register | (cpu::Flags6502::Break as u8) | (cpu::Flags6502::Unused as u8));

    0
}

/// Opcode: Pull Accumulator
pub fn pla(olc: &mut cpu::Olc6502) -> u8 {
    olc.accumulator = olc.read_from_stack();
    olc.set_flag(cpu::Flags6502::Zero, olc.accumulator == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.accumulator & 0x80) != 0);
    0
}

/// Opcode: Pull Processor Status
pub fn plp(olc: &mut cpu::Olc6502) -> u8 {
    olc.status_register = (olc.read_from_stack() | 0x30) - 0x10;

    0
}

/// Opcode: Rotate Left
pub fn rol(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    let result = (olc.fetched_data << 1) | olc.get_flag(cpu::Flags6502::CarryBit);

    olc.set_flag(cpu::Flags6502::CarryBit, (olc.fetched_data & 0b10000000) != 0);
    olc.set_flag(cpu::Flags6502::Zero, result == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    match opcode_table::OPCODE_TABLE[olc.opcode as usize].3 {
        address_modes::AddressMode::Imp => olc.accumulator = result,
        _ => olc.write(olc.addr_abs, result)
    };

    0
}

/// Opcode: Rotate Right
pub fn ror(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    let result = (olc.get_flag(cpu::Flags6502::CarryBit) << 7) | (olc.fetched_data >> 1);

    olc.set_flag(cpu::Flags6502::CarryBit, (olc.fetched_data & 0b00000001) != 0);
    olc.set_flag(cpu::Flags6502::Zero, result == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    match opcode_table::OPCODE_TABLE[olc.opcode as usize].3 {
        address_modes::AddressMode::Imp => olc.accumulator = result,
        _ => olc.write(olc.addr_abs, result)
    };

    0
}

/// Opcode: Return from Interrupt
pub fn rti(olc: &mut cpu::Olc6502) -> u8 {
    olc.status_register = (olc.read_from_stack() | 0x30) - 0x10;
    olc.program_counter = olc.read_counter_from_stack();

    0
}

/// Opcode: Return from Subroutine
pub fn rts(olc: &mut cpu::Olc6502) -> u8 {
    olc.program_counter = olc.read_counter_from_stack();
    olc.program_counter = olc.program_counter.wrapping_add(1);

    0
}

pub fn sbc(olc: &mut cpu::Olc6502) -> u8 {
    olc.fetch();

    // Add in u16 space so we can get any carry bits
    let (result1, overflow1) = olc.accumulator.overflowing_sub(olc.fetched_data);
    let (result, overflow2) = result1.overflowing_sub(1 - olc.get_flag(cpu::Flags6502::CarryBit));
    let overflow = (olc.accumulator ^ olc.fetched_data) & 0x80 != 0 && (olc.accumulator ^ result) & 0x80 != 0;

    olc.set_flag(cpu::Flags6502::CarryBit, !(overflow1 | overflow2));
    olc.set_flag(cpu::Flags6502::Zero, (result & 0x00FF) == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, result > 0x7F);
    olc.set_flag(cpu::Flags6502::Overflow, overflow);

    olc.accumulator = (result & 0x00FF) as u8;

    1
}

/// Opcode: Set Carry Flag
pub fn sec(olc: &mut cpu::Olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::CarryBit, true);
    0
}

/// Opcode: Set Decimal Flag
pub fn sed(olc: &mut cpu::Olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::DecimalMode, true);
    0
}

/// Opcode: Set Interrupt Disable
pub fn sei(olc: &mut cpu::Olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::DisableInterrupts, true);
    0
}

/// Opcode: Store Accumulator
pub fn sta(olc: &mut cpu::Olc6502) -> u8 {
    olc.write(olc.addr_abs, olc.accumulator);
    0
}

/// Opcode: Store X Register
pub fn stx(olc: &mut cpu::Olc6502) -> u8 {
    olc.write(olc.addr_abs, olc.x_register);
    0
}

/// Opcode: Store Y Register
pub fn sty(olc: &mut cpu::Olc6502) -> u8 {
    olc.write(olc.addr_abs, olc.y_register);
    0
}

/// Opcode: Transfer Accumulator to X
pub fn tax(olc: &mut cpu::Olc6502) -> u8 {
    olc.x_register = olc.accumulator;
    olc.set_flag(cpu::Flags6502::Zero, olc.x_register == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.x_register & 0x80) != 0);

    0
}

/// Opcode: Transfer Accumulator to Y
pub fn tay(olc: &mut cpu::Olc6502) -> u8 {
    olc.y_register = olc.accumulator;
    olc.set_flag(cpu::Flags6502::Zero, olc.y_register == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.y_register & 0x80) != 0);

    0
}

/// Opcode: Transfer Stack Pointer to X
pub fn tsx(olc: &mut cpu::Olc6502) -> u8 {
    olc.x_register = olc.stack_pointer;
    olc.set_flag(cpu::Flags6502::Zero, olc.x_register == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.x_register & 0x80) != 0);

    0
}

/// Opcode: Transfer X to Accumulator
pub fn txa(olc: &mut cpu::Olc6502) -> u8 {
    olc.accumulator = olc.x_register;
    olc.set_flag(cpu::Flags6502::Zero, olc.accumulator == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.accumulator & 0x80) != 0);

    0
}

/// Opcode: Transfer X to Stack Pointer
pub fn txs(olc: &mut cpu::Olc6502) -> u8 {
    olc.stack_pointer = olc.x_register;
    0
}

/// Opcode: Transfer Y to Accumulator
pub fn tya(olc: &mut cpu::Olc6502) -> u8 {
    olc.accumulator = olc.y_register;
    olc.set_flag(cpu::Flags6502::Zero, olc.accumulator == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.accumulator & 0x80) != 0);

    0
}

/// This is used when it is an illegal opcode and does nothing
pub fn xxx(_olc: &mut cpu::Olc6502) -> u8 {
    0
}

fn branch_if_set(olc: &mut cpu::Olc6502, flag: cpu::Flags6502) {
    branch_if_flag_equal(olc, flag, 1);
}

fn branch_if_clear(olc: &mut cpu::Olc6502, flag: cpu::Flags6502) {
    branch_if_flag_equal(olc, flag, 0);
}

fn branch_if_flag_equal(olc: &mut cpu::Olc6502, flag: cpu::Flags6502, value: u8) {
    if olc.get_flag(flag) != value {
        return;
    }

    olc.addr_abs = olc.program_counter.wrapping_add(olc.addr_rel);
    olc.cycles += 1;

    // If the addition caused paging, add another cycle
    if (olc.addr_abs & 0xFF00) != (olc.program_counter & 0xFF00) {
        olc.cycles += 1;
    }

    olc.program_counter = olc.addr_abs;
}