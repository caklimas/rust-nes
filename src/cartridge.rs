use std::fs;

pub struct Cartridge {
    prg_memory: Vec<u8>,
    chr_memory: Vec<u8>,
    mapper_id: u8,
    prg_banks: u8,
    chr_banks: u8
}

impl Cartridge {

    /// An iNES file consists of the following sections, in order:
    /// Header (16 bytes)
    /// Trainer, if present (0 or 512 bytes)
    /// PRG ROM data (16384 * x bytes)
    /// CHR ROM data, if present (8192 * y bytes)
    /// PlayChoice INST-ROM, if present (0 or 8192 bytes)
    /// PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing, see PC10 ROM-Images for details)
    pub fn new(file_name: &str) -> Self {
        let bytes = fs::read(file_name).expect("Cannot find file");
        let header = CartridgeHeader::new(&bytes);

        let prg_memory_size = ((header.prg_rom_chunks as u16) * 16384) as usize;
        let post_header_index = if (header.mapper_1 & 0x04) > 0 { 16 + 512 } else { 16 };

        let chr_memory_size = ((header.chr_rom_chunks as u16) * 8192) as usize;
        let chr_memory_start = (post_header_index + prg_memory_size + 1) as usize;

        Cartridge {
            prg_memory: bytes[post_header_index..(post_header_index + prg_memory_size)].to_vec(),
            chr_memory: bytes[chr_memory_start..(chr_memory_start + chr_memory_size)].to_vec(),
            mapper_id: ((header.mapper_2 >> 4) << 4) | (header.mapper_1 >> 4),
            prg_banks: header.prg_rom_chunks,
            chr_banks: header.chr_rom_chunks
        }
    }

    /// Read from the Main Bus
    pub fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool {
        false
    }

    /// Write to the Main Bus
    pub fn cpu_write(&mut self, address: u16, data: u8) -> bool {
        false
    }

    /// Read from the PPU Bus
    pub fn ppu_read(&mut self, address: u16, data: &mut u8) -> bool {
        false
    }

    /// WRite to the PPU Bus
    pub fn ppu_write(&mut self, address: u16, data: u8) -> bool {
        false
    }
}

pub struct CartridgeHeader {
    name: [u8; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper_1: u8,
    mapper_2: u8,
    prg_ram_size: u8,
    tv_system_1: u8,
    tv_system_2: u8,
    unused: [u8; 5]
}

impl CartridgeHeader {
    pub fn new(bytes: &Vec<u8>) -> Self {
        let mut name: [u8; 4] = Default::default();
        name.copy_from_slice(&bytes[0..3]);

        let mut unused: [u8; 5] = Default::default();
        unused.copy_from_slice(&bytes[11..15]);

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