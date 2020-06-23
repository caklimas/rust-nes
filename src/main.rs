mod memory;
mod bus;
mod cpu;
mod opcodes;
mod address_modes;
mod opcode_table;
mod ppu;
mod cartridge;
mod mappers;

fn main() {
    let mut bus = bus::Bus::new();
    let cartridge = cartridge::Cartridge::new(r".\src\test_roms\nestest.nes");
    bus.load_cartridge(cartridge);
    bus.reset();
}