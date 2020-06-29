mod memory;
mod bus;
mod cpu;
mod opcodes;
mod address_modes;
mod opcode_table;
mod ppu;
mod cartridge;
mod mappers;
mod memory_sizes;

fn main() {
    let mut bus = bus::Bus::new();
    bus.reset();
    let cartridge = cartridge::Cartridge::new(r".\src\test_roms\nestest.nes");
    bus.load_cartridge(cartridge);
}