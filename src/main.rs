mod bus;
mod cpu;
mod opcodes;
mod address_modes;
mod opcode_table;

fn main() {
    let mut c = cpu::olc6502::new();
    c.clock();

    let x: i8 = 127;
    let y: i8 = x.wrapping_add(4);
    println!("{}",  0b11111111 ^ 0xFF);

    test_sub();
}

fn test_sub() {
    let address = 0;
    let mut olc = cpu::olc6502::new();
    olc.set_flag(cpu::Flags6502::CarryBit, true);
    olc.opcode = 0xF1;
    olc.addr_abs = address;
    olc.bus.ram[address as usize] = 5;
    olc.accumulator = 3;

    opcodes::sbc(&mut olc);
}
