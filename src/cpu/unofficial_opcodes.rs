use super::{Cpu6502};
use super::opcodes::*;

pub fn dcp(cpu: &mut Cpu6502) -> u8 {
    dec(cpu);
    cmp(cpu)
}

pub fn isc(cpu: &mut Cpu6502) -> u8 {
    inc(cpu);
    sbc(cpu)
}

pub fn lax(cpu: &mut Cpu6502) -> u8 {
    lda(cpu);
    tax(cpu)
}

pub fn rla(cpu: &mut Cpu6502) -> u8 {
    rol(cpu);
    and(cpu)
}

pub fn rra(cpu: &mut Cpu6502) -> u8 {
    ror(cpu);
    adc(cpu)
}

pub fn sax(cpu: &mut Cpu6502) -> u8 {
    cpu.write(cpu.addr_abs, cpu.accumulator & cpu.x_register);
    0
}

pub fn slo(cpu: &mut Cpu6502) -> u8 {
    asl(cpu);
    ora(cpu)
}

pub fn sre(cpu: &mut Cpu6502) -> u8 {
    lsr(cpu);
    eor(cpu)
}

pub fn stp(cpu: &mut Cpu6502) -> u8 {
    panic!("Illegal opcode: 0x{:02X}", cpu.opcode);
}

/// This is used when it is an illegal opcode and does nothing
pub fn xxx(cpu: &mut Cpu6502) -> u8 {
    // println!("Unofficial opcode 0x{:02X}", cpu.opcode);
    0
}