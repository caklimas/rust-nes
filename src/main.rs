mod bus;
mod cpu;
mod opcodes;
mod address_modes;
mod opcode_table;
mod ppu;
mod cartridge;

fn main() {
    let mut bus = bus::Bus::new();
    bus.clock();

    let x: i8 = 127;
    let y: i8 = x.wrapping_add(4);
    println!("{:b}",  0b10100000 & (1 << 7));
}
