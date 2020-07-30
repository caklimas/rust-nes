use std::fs;
use crate::memory_sizes::KILOBYTES_16;
use crate::mappers;
use super::cartridge_header::CartridgeHeader;
use super::mirror::Mirror;

pub struct Cartridge {
    prg_memory: Vec<u8>,
    chr_memory: Vec<u8>,
    mapper_id: u8,
    prg_banks: u8,
    chr_banks: u8,
    mapper: Option<Box<dyn mappers::mappers::Mapper>>,
    mirror: Mirror
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
        let mapper_id = ((header.mapper_2 >> 4) << 4) | (header.mapper_1 >> 4);
        let mirror = if (header.mapper_1 & 0x01) > 0 { Mirror::Vertical } else { Mirror::Horizontal };

        let prg_memory_size = ((header.prg_rom_chunks as u32) * (KILOBYTES_16 as u32)) as usize;
        let post_header_index = if (header.mapper_1 & 0x04) > 0 { 16 + 512 } else { 16 };

        let chr_memory_start = (post_header_index + prg_memory_size) as usize;
        let chr_memory = if header.chr_rom_chunks == 0 { 
            vec![0; 8192]
        } else { 
            let chr_memory_size = ((header.chr_rom_chunks as u32) * 8192) as usize;
            bytes[chr_memory_start..(chr_memory_start + chr_memory_size)].to_vec()
        };

        let prg_memory = bytes[post_header_index..(post_header_index + prg_memory_size)].to_vec();
 
        Cartridge {
            prg_memory,
            chr_memory,
            mapper_id,
            prg_banks: header.prg_rom_chunks,
            chr_banks: header.chr_rom_chunks,
            mapper: Cartridge::get_mapper(mapper_id, header.prg_rom_chunks, header.chr_rom_chunks),
            mirror
        }
    }

    pub fn reset(&mut self) {
        match self.mapper {
            Some(ref mut m) => {
                m.reset()
            },
            None => ()
        };
    }

    /// Read from the Main Bus
    pub fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool {
        let mut mapped_address: u32 = 0;
        match self.mapper {
            Some(ref mut m) => {
                if m.cpu_map_read(address, &mut mapped_address) {
                    *data = self.prg_memory[mapped_address as usize];
                    return true;
                }
            },
            None => ()
        };

        false
    }

    /// Write to the Main Bus
    pub fn cpu_write(&mut self, address: u16, data: u8) -> bool {
        let mut mapped_address: u32 = 0;
        match self.mapper {
            Some(ref mut m) => {
                if m.cpu_map_write(address, &mut mapped_address, data) {
                    self.prg_memory[mapped_address as usize] = data;
                    return true;
                }
            },
            None => ()
        };
        
        false
    }

    /// Read from the PPU Bus
    pub fn ppu_read(&mut self, address: u16, data: &mut u8) -> bool {
        let mut mapped_address: u32 = 0;

        match self.mapper {
            Some(ref mut m) => {
                if m.ppu_map_read(address, &mut mapped_address) {
                    *data = self.chr_memory[mapped_address as usize];
                    return true;
                }
            },
            None => ()
        };

        false
    }

    /// Write to the PPU Bus
    pub fn ppu_write(&mut self, address: u16, data: u8) -> bool {
        let mut mapped_address: u32 = 0;
        match self.mapper {
            Some(ref mut m) => {
                if m.ppu_map_write(address, &mut mapped_address, data) {
                    self.chr_memory[mapped_address as usize] = data;
                    return true;
                }
            },
            None => ()
        };

        false
    }

    pub fn get_mirror(&mut self) -> Mirror {
        match self.mapper {
            Some(ref mut mapper) => {
                let mirror = mapper.get_mirror();
                match mirror {
                    Mirror::Hardware => self.mirror,
                    _ => mirror
                }
            },
            None => self.mirror
        }
    }

    fn get_mapper(mapper_id: u8, prg_banks: u8, chr_banks: u8) -> Option<Box<dyn mappers::mappers::Mapper>> {
        let mut mapper: Option<Box<dyn mappers::mappers::Mapper>> = None;
        match mapper_id {
            0 => mapper = Some(Box::new(mappers::mapper000::Mapper000::new(prg_banks, chr_banks))),
            2 => mapper = Some(Box::new(mappers::mapper002::Mapper002::new(prg_banks, chr_banks))),
            3 => mapper = Some(Box::new(mappers::mapper003::Mapper003::new(prg_banks, chr_banks))),
            _ => ()
        };

        mapper
    }
}

impl CartridgeHeader {
    pub fn new(bytes: &Vec<u8>) -> Self {
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