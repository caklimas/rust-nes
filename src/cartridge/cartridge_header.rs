pub struct CartridgeHeader {
    pub name: [u8; 4],
    pub prg_rom_chunks: u8,
    pub chr_rom_chunks: u8,
    pub mapper_1: u8,
    pub mapper_2: u8,
    pub prg_ram_size: u8,
    pub tv_system_1: u8,
    pub tv_system_2: u8,
    pub unused: [u8; 5]
}