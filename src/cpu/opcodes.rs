use crate::cpu::cpu;
use crate::cpu::opcode_table;
use crate::cpu::address_modes;


/// Opcode: Add with Carry
/// Overflow occurs when you add two positive numbers together and get a negative or you add two negative and get a positive
/// To check this you check the most significant bits of accumulator, memory and result
/// In order for overflow to occur, the most significant bits of the accumulator and memory need to be the same and the result needs to be different
/// We can do this using XOR
pub fn adc(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    // Add in u16 space so we can get any carry bits
    let (result1, overflow1) = cpu6502.fetched_data.overflowing_add(cpu6502.accumulator);
    let (result, overflow2) = result1.overflowing_add(cpu6502.get_flag(cpu::Flags6502::CarryBit));
    let overflow = (cpu6502.accumulator ^ cpu6502.fetched_data) & 0x80 == 0 && (cpu6502.accumulator ^ result) & 0x80 != 0;

    cpu6502.set_flag(cpu::Flags6502::CarryBit, overflow1 | overflow2);
    cpu6502.set_flag(cpu::Flags6502::Zero, (result & 0x00FF) == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);
    cpu6502.set_flag(cpu::Flags6502::Overflow, overflow);

    cpu6502.accumulator = (result & 0x00FF) as u8;

    1
}

/// Opcode: Logical AND
pub fn and(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();
    cpu6502.accumulator = cpu6502.accumulator & cpu6502.fetched_data;

    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.accumulator == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.accumulator & 0x80) != 0);

    1
}

/// Opcode: Arithmetic Shift Left
pub fn asl(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    let shifted = (cpu6502.fetched_data as u16) << 1;

    cpu6502.set_flag(cpu::Flags6502::CarryBit, shifted > 0xFF);
    cpu6502.set_flag(cpu::Flags6502::Zero, (shifted & 0x00FF) == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (shifted & 0x80) != 0);

    let result = (shifted as u8) & 0xFF;
    match opcode_table::OPCODE_TABLE[cpu6502.opcode as usize].3 {
        address_modes::AddressMode::Imp => cpu6502.accumulator = result,
        _ => cpu6502.write(cpu6502.addr_abs, result)
    };

    1
}

/// Opcode: Branch if Carry Clear
pub fn bcc(cpu6502: &mut cpu::Cpu6502) -> u8 {
    branch_if_clear(cpu6502, cpu::Flags6502::CarryBit);
    0
}

/// Opcode: Branch if Carry Set
pub fn bcs(cpu6502: &mut cpu::Cpu6502) -> u8 {
    branch_if_set(cpu6502, cpu::Flags6502::CarryBit);
    0
}

/// Opcode: Branch if Equal
pub fn beq(cpu6502: &mut cpu::Cpu6502) -> u8 {
    branch_if_set(cpu6502, cpu::Flags6502::Zero);
    0
}

/// Opcode: Bit Test
pub fn bit(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    let bit6 = (cpu6502.fetched_data & 0x40) >> 6;
    let bit7 = (cpu6502.fetched_data & 0x80) >> 7;
    cpu6502.set_flag(cpu::Flags6502::Zero, (cpu6502.fetched_data & cpu6502.accumulator) == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Overflow, bit6 == 1);
    cpu6502.set_flag(cpu::Flags6502::Negative, bit7 == 1);

    0
}

/// Opcode: Branch if Minus
pub fn bmi(cpu6502: &mut cpu::Cpu6502) -> u8 {
    branch_if_set(cpu6502, cpu::Flags6502::Negative);
    0
}

/// Opcode: Branch if Not Equal
pub fn bne(cpu6502: &mut cpu::Cpu6502) -> u8 {
    branch_if_clear(cpu6502, cpu::Flags6502::Zero);
    0
}

/// Opcode: Branch if Positive
pub fn bpl(cpu6502: &mut cpu::Cpu6502) -> u8 {
    branch_if_clear(cpu6502, cpu::Flags6502::Negative);
    0
}

/// Opcode: Force Interrupt
/// Push program counter and processor status into stack
pub fn brk(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.program_counter += 1;
    cpu6502.set_flag(cpu::Flags6502::Break, true);
    cpu6502.set_flag(cpu::Flags6502::DisableInterrupts, true);

    cpu6502.write_counter_to_stack();
    cpu6502.write_to_stack(cpu6502.status_register);
    cpu6502.program_counter = cpu6502.read_program_counter(cpu::INTERRUPT_PROGRAM_COUNTER_ADDRESS);

    0
}

/// Opcode: Branch if Overflow Clear
pub fn bvc(cpu6502: &mut cpu::Cpu6502) -> u8 {
    branch_if_clear(cpu6502, cpu::Flags6502::Overflow);
    0
}

/// Opcode: Branch if Overflow Set
pub fn bvs(cpu6502: &mut cpu::Cpu6502) -> u8 {
    branch_if_set(cpu6502, cpu::Flags6502::Overflow);
    0
}

/// Opcode: Clear Carry Flag
pub fn clc(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.set_flag(cpu::Flags6502::CarryBit, false);
    0
}

/// Opcode: Clear Decimal Mode
pub fn cld(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.set_flag(cpu::Flags6502::DecimalMode, false);
    0
}

/// Opcode: Clear Interrupt Disable
pub fn cli(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.set_flag(cpu::Flags6502::DisableInterrupts, false);
    0
}

/// Opcode: Clear Overflow Flag
pub fn clv(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.set_flag(cpu::Flags6502::Overflow, false);
    0
}

/// Opcode: Compare
pub fn cmp(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    let result = (cpu6502.accumulator as u16).wrapping_sub(cpu6502.fetched_data as u16);

    cpu6502.set_flag(cpu::Flags6502::CarryBit, cpu6502.accumulator >= cpu6502.fetched_data);
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.accumulator == cpu6502.fetched_data);
    cpu6502.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Compare X Register
pub fn cpx(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    let result = (cpu6502.x_register as u16).wrapping_sub(cpu6502.fetched_data as u16);

    cpu6502.set_flag(cpu::Flags6502::CarryBit, cpu6502.x_register >= cpu6502.fetched_data);
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.x_register == cpu6502.fetched_data);
    cpu6502.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Compare Y Register
pub fn cpy(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    let result = (cpu6502.y_register as u16).wrapping_sub(cpu6502.fetched_data as u16);

    cpu6502.set_flag(cpu::Flags6502::CarryBit, cpu6502.y_register >= cpu6502.fetched_data);
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.y_register == cpu6502.fetched_data);
    cpu6502.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Decrement Memory
pub fn dec(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    let result = cpu6502.fetched_data.wrapping_sub(1);
    cpu6502.write(cpu6502.addr_abs, result & 0x00FF);

    cpu6502.set_flag(cpu::Flags6502::Zero, result == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Decrement X Register
pub fn dex(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.x_register = cpu6502.x_register.wrapping_sub(1);

    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.x_register == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.x_register & 0x80) != 0);

    0
}

/// Opcode: Decrement Y Register
pub fn dey(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.y_register = cpu6502.y_register.wrapping_sub(1);

    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.y_register == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.y_register & 0x80) != 0);

    0
}

/// Opcode: Exclusive OR
pub fn eor(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    cpu6502.accumulator = cpu6502.accumulator ^ cpu6502.fetched_data;

    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.accumulator == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.accumulator & 0x80) != 0);

    1
}

/// Opcode: Increment Memory
pub fn inc(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    let result = cpu6502.fetched_data.wrapping_add(1);
    cpu6502.write(cpu6502.addr_abs, result & 0x00FF);

    cpu6502.set_flag(cpu::Flags6502::Zero, result == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

/// Opcode: Increment X Register
pub fn inx(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.x_register = cpu6502.x_register.wrapping_add(1);

    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.x_register == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.x_register & 0x80) != 0);

    0
}

/// Opcode: Increment Y Register
pub fn iny(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.y_register = cpu6502.y_register.wrapping_add(1);

    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.y_register == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.y_register & 0x80) != 0);

    0
}

/// Opcode: Jump
pub fn jmp(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.program_counter = cpu6502.addr_abs;
    0
}

/// Opcode: Jump to Subroutine
pub fn jsr(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.program_counter -= 1;
    cpu6502.write_counter_to_stack();
    cpu6502.program_counter = cpu6502.addr_abs;

    0
}

/// Opcode: Load Accumulator
pub fn lda(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    cpu6502.accumulator = cpu6502.fetched_data;
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.accumulator == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.accumulator & 0x80) != 0);

    0
}

/// Opcode: Load X Register
pub fn ldx(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    cpu6502.x_register = cpu6502.fetched_data;
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.x_register == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.x_register & 0x80) != 0);

    0
}

/// Opcode: Load Y Register
pub fn ldy(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    cpu6502.y_register = cpu6502.fetched_data;
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.y_register == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.y_register & 0x80) != 0);

    0
}

/// Opcode: Logical Shift Right
pub fn lsr(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    let shifted = cpu6502.fetched_data >> 1;
    cpu6502.set_flag(cpu::Flags6502::Zero, shifted == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (shifted & 0x80) != 0);
    cpu6502.set_flag(cpu::Flags6502::CarryBit, (cpu6502.fetched_data & 0x0001) != 0);
    
    let result = (shifted as u8) & 0xFF;
    match opcode_table::OPCODE_TABLE[cpu6502.opcode as usize].3 {
        address_modes::AddressMode::Imp => cpu6502.accumulator = result,
        _ => cpu6502.write(cpu6502.addr_abs, result)
    };

    0
}

/// Opcode: No Operation
pub fn nop(cpu6502: &mut cpu::Cpu6502) -> u8 {
    match cpu6502.opcode {
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
pub fn ora(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    cpu6502.accumulator = cpu6502.accumulator | cpu6502.fetched_data;
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.accumulator == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.accumulator & 0x80) != 0);

    1
}

/// Opcode: Push Accumulator
pub fn pha(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.write_to_stack(cpu6502.accumulator);
    0
}

/// Opcode: Push Processor Status
pub fn php(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.write_to_stack(cpu6502.status_register | (cpu::Flags6502::Break as u8) | (cpu::Flags6502::Unused as u8));

    0
}

/// Opcode: Pull Accumulator
pub fn pla(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.accumulator = cpu6502.read_from_stack();
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.accumulator == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.accumulator & 0x80) != 0);
    0
}

/// Opcode: Pull Processor Status
pub fn plp(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.status_register = (cpu6502.read_from_stack() | 0x30) - 0x10;

    0
}

/// Opcode: Rotate Left
pub fn rol(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    let result = (cpu6502.fetched_data << 1) | cpu6502.get_flag(cpu::Flags6502::CarryBit);

    cpu6502.set_flag(cpu::Flags6502::CarryBit, (cpu6502.fetched_data & 0b10000000) != 0);
    cpu6502.set_flag(cpu::Flags6502::Zero, result == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    match opcode_table::OPCODE_TABLE[cpu6502.opcode as usize].3 {
        address_modes::AddressMode::Imp => cpu6502.accumulator = result,
        _ => cpu6502.write(cpu6502.addr_abs, result)
    };

    0
}

/// Opcode: Rotate Right
pub fn ror(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    let result = (cpu6502.get_flag(cpu::Flags6502::CarryBit) << 7) | (cpu6502.fetched_data >> 1);

    cpu6502.set_flag(cpu::Flags6502::CarryBit, (cpu6502.fetched_data & 0b00000001) != 0);
    cpu6502.set_flag(cpu::Flags6502::Zero, result == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    match opcode_table::OPCODE_TABLE[cpu6502.opcode as usize].3 {
        address_modes::AddressMode::Imp => cpu6502.accumulator = result,
        _ => cpu6502.write(cpu6502.addr_abs, result)
    };

    0
}

/// Opcode: Return from Interrupt
pub fn rti(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.status_register = (cpu6502.read_from_stack() | 0x30) - 0x10;
    cpu6502.program_counter = cpu6502.read_counter_from_stack();

    0
}

/// Opcode: Return from Subroutine
pub fn rts(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.program_counter = cpu6502.read_counter_from_stack();
    cpu6502.program_counter = cpu6502.program_counter.wrapping_add(1);

    0
}

pub fn sbc(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.fetch();

    // Add in u16 space so we can get any carry bits
    let (result1, overflow1) = cpu6502.accumulator.overflowing_sub(cpu6502.fetched_data);
    let (result, overflow2) = result1.overflowing_sub(1 - cpu6502.get_flag(cpu::Flags6502::CarryBit));
    let overflow = (cpu6502.accumulator ^ cpu6502.fetched_data) & 0x80 != 0 && (cpu6502.accumulator ^ result) & 0x80 != 0;

    cpu6502.set_flag(cpu::Flags6502::CarryBit, !(overflow1 | overflow2));
    cpu6502.set_flag(cpu::Flags6502::Zero, (result & 0x00FF) == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, result > 0x7F);
    cpu6502.set_flag(cpu::Flags6502::Overflow, overflow);

    cpu6502.accumulator = (result & 0x00FF) as u8;

    1
}

/// Opcode: Set Carry Flag
pub fn sec(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.set_flag(cpu::Flags6502::CarryBit, true);
    0
}

/// Opcode: Set Decimal Flag
pub fn sed(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.set_flag(cpu::Flags6502::DecimalMode, true);
    0
}

/// Opcode: Set Interrupt Disable
pub fn sei(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.set_flag(cpu::Flags6502::DisableInterrupts, true);
    0
}

/// Opcode: Store Accumulator
pub fn sta(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.write(cpu6502.addr_abs, cpu6502.accumulator);
    0
}

/// Opcode: Store X Register
pub fn stx(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.write(cpu6502.addr_abs, cpu6502.x_register);
    0
}

/// Opcode: Store Y Register
pub fn sty(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.write(cpu6502.addr_abs, cpu6502.y_register);
    0
}

/// Opcode: Transfer Accumulator to X
pub fn tax(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.x_register = cpu6502.accumulator;
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.x_register == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.x_register & 0x80) != 0);

    0
}

/// Opcode: Transfer Accumulator to Y
pub fn tay(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.y_register = cpu6502.accumulator;
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.y_register == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.y_register & 0x80) != 0);

    0
}

/// Opcode: Transfer Stack Pointer to X
pub fn tsx(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.x_register = cpu6502.stack_pointer;
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.x_register == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.x_register & 0x80) != 0);

    0
}

/// Opcode: Transfer X to Accumulator
pub fn txa(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.accumulator = cpu6502.x_register;
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.accumulator == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.accumulator & 0x80) != 0);

    0
}

/// Opcode: Transfer X to Stack Pointer
pub fn txs(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.stack_pointer = cpu6502.x_register;
    0
}

/// Opcode: Transfer Y to Accumulator
pub fn tya(cpu6502: &mut cpu::Cpu6502) -> u8 {
    cpu6502.accumulator = cpu6502.y_register;
    cpu6502.set_flag(cpu::Flags6502::Zero, cpu6502.accumulator == 0x00);
    cpu6502.set_flag(cpu::Flags6502::Negative, (cpu6502.accumulator & 0x80) != 0);

    0
}

/// This is used when it is an illegal opcode and does nothing
pub fn xxx(_cpu6502: &mut cpu::Cpu6502) -> u8 {
    0
}

fn branch_if_set(cpu6502: &mut cpu::Cpu6502, flag: cpu::Flags6502) {
    branch_if_flag_equal(cpu6502, flag, 1);
}

fn branch_if_clear(cpu6502: &mut cpu::Cpu6502, flag: cpu::Flags6502) {
    branch_if_flag_equal(cpu6502, flag, 0);
}

fn branch_if_flag_equal(cpu6502: &mut cpu::Cpu6502, flag: cpu::Flags6502, value: u8) {
    if cpu6502.get_flag(flag) != value {
        return;
    }

    cpu6502.addr_abs = cpu6502.program_counter.wrapping_add(cpu6502.addr_rel);
    cpu6502.cycles += 1;

    // If the addition caused paging, add another cycle
    if (cpu6502.addr_abs & 0xFF00) != (cpu6502.program_counter & 0xFF00) {
        cpu6502.cycles += 1;
    }

    cpu6502.program_counter = cpu6502.addr_abs;
}