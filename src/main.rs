mod memory;
mod bus;
mod cpu;
mod opcodes;
mod address_modes;
mod opcode_table;
mod ppu;
mod cartridge;

fn main() {
    let mut bus = bus::Bus::new();

    let x = [1,2,3,4,5];
    let mut z = 5;
    something(&mut z);

    println!("{}", z);
}

fn something(n: &mut u8) {
    *n += 1;
}