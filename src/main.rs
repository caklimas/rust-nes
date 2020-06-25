mod memory;
mod bus;
mod cpu;
mod opcodes;
mod address_modes;
mod opcode_table;
mod ppu;
mod cartridge;
mod mappers;
mod snake;

fn main() {
    let mut bus = bus::Bus::new();
    let cartridge = cartridge::Cartridge::new(r".\src\test_roms\nestest.nes");
    bus.load_cartridge(cartridge);

    let mut x = 0;
    while x < 26555 {
        bus.clock();
        x += 1;
    }

    let m = bus.memory.borrow_mut().read(0x02, false);
    println!("{}", m);
}