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

impl CartridgeHeader {
    pub fn new(bytes: &[u8]) -> Self {
        let mut name: [u8; 4] = Default::default();
        name.copy_from_slice(&bytes[0..4]);

        let mut unused: [u8; 5] = Default::default();
        unused.copy_from_slice(&bytes[11..16]);

        CartridgeHeader {
            name,
            prg_rom_chunks: bytes[4],
            chr_rom_chunks: bytes[5],
            mapper_1: bytes[6],
            mapper_2: bytes[7],
            prg_ram_size: bytes[8],
            tv_system_1: bytes[9],
            tv_system_2: bytes[10],
            unused
        }
    }
}