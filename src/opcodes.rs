use crate::cpu;
use crate::opcode_table;
use crate::address_modes;

/// Opcode: Add with Carry
/// Overflow occurs when you add two positive numbers together and get a negative or you add two negative and get a positive
/// To check this you check the most significant bits of accumulator, memory and result
/// In order for overflow to occur, the most significant bits of the accumulator and memory need to be the same and the result needs to be different
/// We can do this using XOR
pub fn adc(olc: &mut cpu::olc6502) -> u8 {
    olc.fetch();

    // Add in u16 space so we can get any carry bits
    let result = (olc.fetched_data as u16).wrapping_add(olc.accumulator as u16).wrapping_add(olc.get_flag(cpu::Flags6502::CarryBit) as u16);

    olc.set_flag(cpu::Flags6502::CarryBit, result > 0xFF);
    olc.set_flag(cpu::Flags6502::Zero, (result & 0x00FF) == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.accumulator & 0x80) != 0);
    olc.set_flag(cpu::Flags6502::Overflow, is_overflow(olc, result));

    olc.accumulator = (result & 0x00FF) as u8;

    1
}

/// Opcode: Logical AND
pub fn and(olc: &mut cpu::olc6502) -> u8 {
    olc.fetch();
    olc.accumulator = olc.accumulator & olc.fetched_data;

    olc.set_flag(cpu::Flags6502::Zero, olc.accumulator == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.accumulator & 0x80) != 0);

    1
}

/// Opcode: Arithmetic Shift Left
pub fn asl(olc: &mut cpu::olc6502) -> u8 {
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
pub fn bcc(olc: &mut cpu::olc6502) -> u8 {
    branch_if_clear(olc, cpu::Flags6502::CarryBit);
    0
}

/// Opcode: Branch if Carry Set
pub fn bcs(olc: &mut cpu::olc6502) -> u8 {
    branch_if_set(olc, cpu::Flags6502::CarryBit);
    0
}

/// Opcode: Branch if Equal
pub fn beq(olc: &mut cpu::olc6502) -> u8 {
    branch_if_set(olc, cpu::Flags6502::Zero);
    0
}

/// Opcode: Bit Test
pub fn bit(olc: &mut cpu::olc6502) -> u8 {
    olc.fetch();

    let bit6 = (olc.fetched_data & 0x40) >> 6;
    let bit7 = (olc.fetched_data & 0x80) >> 7;
    olc.set_flag(cpu::Flags6502::Zero, (olc.fetched_data & olc.accumulator) == 0x00);
    olc.set_flag(cpu::Flags6502::Overflow, bit6 == 1);
    olc.set_flag(cpu::Flags6502::Negative, bit7 == 1);

    0
}

/// Opcode: Branch if Minus
pub fn bmi(olc: &mut cpu::olc6502) -> u8 {
    branch_if_set(olc, cpu::Flags6502::Negative);
    0
}

/// Opcode: Branch if Not Equal
pub fn bne(olc: &mut cpu::olc6502) -> u8 {
    branch_if_clear(olc, cpu::Flags6502::Zero);
    0
}

/// Opcode: Branch if Positive
pub fn bpl(olc: &mut cpu::olc6502) -> u8 {
    branch_if_clear(olc, cpu::Flags6502::Negative);
    0
}

/// Opcode: Force Interrupt
/// Push program counter and processor status into stack
pub fn brk(olc: &mut cpu::olc6502) -> u8 {
    olc.program_counter += 1;
    olc.set_flag(cpu::Flags6502::Break, true);

    olc.write_to_stack(((olc.program_counter >> 8) & 0x00FF) as u8);
    olc.write_to_stack((olc.program_counter & 0x00FF) as u8);
    olc.write_to_stack(olc.status_register);

    olc.program_counter = (olc.read(0xFFFF, false) as u16) << 8 | olc.read(0xFFFE, false) as u16;
    0
}

/// Opcode: Branch if Overflow Clear
pub fn bvc(olc: &mut cpu::olc6502) -> u8 {
    branch_if_clear(olc, cpu::Flags6502::Overflow);
    0
}

/// Opcode: Branch if Overflow Set
pub fn bvs(olc: &mut cpu::olc6502) -> u8 {
    branch_if_set(olc, cpu::Flags6502::Overflow);
    0
}

/// Opcode: Clear Carry Flag
pub fn clc(olc: &mut cpu::olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::CarryBit, false);
    0
}

/// Opcode: Clear Decimal Mode
pub fn cld(olc: &mut cpu::olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::DecimalMode, false);
    0
}

/// Opcode: Clear Interrupt Disable
pub fn cli(olc: &mut cpu::olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::DisableInterrupts, false);
    0
}

/// Opcode: Clear Overflow Flag
pub fn clv(olc: &mut cpu::olc6502) -> u8 {
    olc.set_flag(cpu::Flags6502::Overflow, false);
    0
}

pub fn cmp(olc: &mut cpu::olc6502) -> u8 {
    olc.fetch();

    let result = (olc.accumulator as u16).wrapping_sub(olc.fetched_data as u16);

    olc.set_flag(cpu::Flags6502::CarryBit, olc.accumulator >= olc.fetched_data);
    olc.set_flag(cpu::Flags6502::Zero, olc.accumulator == olc.fetched_data);
    olc.set_flag(cpu::Flags6502::Negative, (result & 0x80) != 0);

    0
}

pub fn cpx(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn cpy(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn dec(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn dex(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn dey(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn eor(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn inc(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn inx(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn iny(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn jmp(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn jsr(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn lda(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn ldx(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn ldy(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn lsr(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn nop(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn ora(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn pha(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn php(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn pla(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn plp(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn rol(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn ror(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn rti(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn rts(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn sbc(olc: &mut cpu::olc6502) -> u8 {
    olc.fetch();

    // Add in u16 space so we can get any carry bits
    let result = (olc.fetched_data as u16).wrapping_sub(olc.accumulator as u16).wrapping_sub((1 - olc.get_flag(cpu::Flags6502::CarryBit)) as u16);
    println!("{}", (result & 0x00FF));
    
    olc.set_flag(cpu::Flags6502::CarryBit, result > 0xFF);
    olc.set_flag(cpu::Flags6502::Zero, (result & 0x00FF) == 0x00);
    olc.set_flag(cpu::Flags6502::Negative, (olc.accumulator & 0x80) != 0);
    olc.set_flag(cpu::Flags6502::Overflow, is_overflow(olc, result));

    olc.accumulator = (result & 0x00FF) as u8;

    1
}

pub fn sec(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn sed(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn sei(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn sta(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn stx(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn sty(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn tax(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn tay(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn tsx(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn txa(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn txs(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn tya(olc: &mut cpu::olc6502) -> u8 {
    0
}

pub fn xxx(olc: &mut cpu::olc6502) -> u8 {
    0
}

fn is_overflow(olc: &cpu::olc6502, result: u16) -> bool {
    let data_accum_same_bits =  ((olc.fetched_data & 0x80) as u16) ^ ((olc.accumulator & 0x80) as u16) != 0x80;
    let data_result_diff_bits = ((olc.fetched_data & 0x80) as u16) ^ (result & 0x80) == 0x80;

    return data_accum_same_bits && data_result_diff_bits;
}

fn branch_if_set(olc: &mut cpu::olc6502, flag: cpu::Flags6502) {
    branch_if_flag_equal(olc, flag, 1);
}

fn branch_if_clear(olc: &mut cpu::olc6502, flag: cpu::Flags6502) {
    branch_if_flag_equal(olc, flag, 0);
}

fn branch_if_flag_equal(olc: &mut cpu::olc6502, flag: cpu::Flags6502, value: u8) {
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

#[cfg(test)]
mod tests {
    use crate::cpu;
    use crate::opcodes::*;

    #[test]
    fn adc_sets_carry_flag_correctly() {
        let address = 0;
        let mut olc = cpu::olc6502::new();
        olc.opcode = 0x79;
        olc.addr_abs = address;
        olc.bus.ram[address as usize] = 255;
        olc.accumulator = 1;

        adc(&mut olc);

        assert_eq!(olc.get_flag(cpu::Flags6502::CarryBit), 1);
    }

    #[test]
    fn adc_sets_zero_flag_correctly() {
        let address = 0;
        let mut olc = cpu::olc6502::new();
        olc.opcode = 0x79;
        olc.addr_abs = address;
        olc.bus.ram[address as usize] = 0;
        olc.accumulator = 0;

        adc(&mut olc);

        assert_eq!(olc.get_flag(cpu::Flags6502::Zero), 1);
    }

    #[test]
    fn adc_sets_negative_flag_correctly() {
        let address = 0;
        let mut olc = cpu::olc6502::new();
        olc.opcode = 0x79;
        olc.addr_abs = address;
        olc.bus.ram[address as usize] = 0;
        olc.accumulator = 0b10000001;

        adc(&mut olc);

        assert_eq!(olc.get_flag(cpu::Flags6502::Negative), 1);
    }

    #[test]
    fn adc_sets_overflow_flag_correctly() {
        let address = 0;
        let mut olc = cpu::olc6502::new();
        olc.opcode = 0x79;
        olc.addr_abs = address;
        olc.bus.ram[address as usize] = 127;
        olc.accumulator = 4;

        adc(&mut olc);

        assert_eq!(olc.get_flag(cpu::Flags6502::Overflow), 1);
    }

    #[test]
    fn adc_adds_correctly() {
        let address = 0;
        let mut olc = cpu::olc6502::new();
        olc.opcode = 0x79;
        olc.addr_abs = address;
        olc.bus.ram[address as usize] = 200;
        olc.accumulator = 4;

        adc(&mut olc);

        assert_eq!(olc.accumulator, 204);
    }

    #[test]
    fn sbc_subtracts_correctly() {
        let address = 0;
        let mut olc = cpu::olc6502::new();
        olc.set_flag(cpu::Flags6502::CarryBit, true);
        olc.opcode = 0xF1;
        olc.addr_abs = address;
        olc.bus.ram[address as usize] = 5;
        olc.accumulator = 3;

        sbc(&mut olc);

        assert_eq!(olc.accumulator, 2);
    }

    #[test]
    fn clc_sets_carry_flag_correctly() {
        let mut olc = cpu::olc6502::new();
        olc.set_flag(cpu::Flags6502::CarryBit, true);
        clc(&mut olc);

        assert_eq!(olc.get_flag(cpu::Flags6502::CarryBit), 0);
    }
}