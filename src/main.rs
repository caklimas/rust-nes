mod memory;
mod bus;
mod cpu;
mod opcodes;
mod address_modes;
mod opcode_table;
mod ppu;
mod cartridge;
mod mappers;
mod display;

fn main() {
    let mut bus = bus::Bus::new();
    let cartridge = cartridge::Cartridge::new(r".\src\test_roms\nestest.nes");
    bus.load_cartridge(cartridge);

    let mut tile = display::Tile {
        planes: [
            [0x41, 0xC2, 0x44, 0x48, 0x10, 0x20, 0x40, 0x80],
            [0x01, 0x02, 0x04, 0x08, 0x16, 0x21, 0x42, 0x87]
        ]
    };

    let c = tile.get_colors();

    println!("{:?}", c);
}