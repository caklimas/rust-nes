mod bus;
mod cpu;
mod opcodes;
mod address_modes;
mod opcode_table;

fn main() {
    let mut c = cpu::olc6502::new();
    c.clock();
}
