pub mod cartridge_header;
pub mod mirror;

use std::fs;
use crate::memory_sizes::KILOBYTES_16;
use crate::mappers;

pub struct Cartridge {
    pub mapper: Option<Box<dyn mappers::mapper::Mapper>>,
    chr_banks: u8,
    chr_memory: Vec<u8>,
    file_path: std::string::String,
    mapper_id: u8,
    mirror: mirror::Mirror,
    prg_banks: u8,
    prg_memory: Vec<u8>
}

impl Cartridge {

    /// An iNES file consists of the following sections, in order:
    /// Header (16 bytes)
    /// Trainer, if present (0 or 512 bytes)
    /// PRG ROM data (16384 * x bytes)
    /// CHR ROM data, if present (8192 * y bytes)
    /// PlayChoice INST-ROM, if present (0 or 8192 bytes)
    /// PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing, see PC10 ROM-Images for details)
    pub fn new(file_path: &str) -> Self {
        let bytes = fs::read(file_path).expect("Cannot find file");
        let header = cartridge_header::CartridgeHeader::new(&bytes);
        let mapper_id = ((header.mapper_2 >> 4) << 4) | (header.mapper_1 >> 4);
        let mirror = if (header.mapper_1 & 0x01) > 0 { mirror::Mirror::Vertical } else { mirror::Mirror::Horizontal };

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
            chr_banks: header.chr_rom_chunks,
            chr_memory,
            file_path: file_path.to_owned(),
            prg_banks: header.prg_rom_chunks,
            prg_memory,
            mapper: Cartridge::get_mapper(mapper_id, &header, file_path),
            mapper_id,
            mirror
        }
    }

    pub fn reset(&mut self) {
        if let Some(ref mut m) = self.mapper {
            m.reset();
        }
    }

    /// Read from the Main Bus
    pub fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool {
        if let Some(ref mut m) = self.mapper {
            let result = m.cpu_map_read(address);
            if result.read_from_cart_ram {
                *data = self.prg_memory[result.mapped_address as usize];
                return true;
            } else if result.read_from_mapper_ram {
                *data = result.data;
                return true;
            }
        }

        false
    }

    /// Write to the Main Bus
    pub fn cpu_write(&mut self, address: u16, data: u8) -> bool {
        if let Some(ref mut m) = self.mapper {
            let result = m.cpu_map_write(address, data);
            if result.write_to_cart_ram {
                self.prg_memory[result.mapped_address as usize] = data;
            }

            if result.handled {
                return true;
            }
        }

        false
    }

    /// Read from the PPU Bus
    pub fn ppu_read(&mut self, address: u16, data: &mut u8) -> bool {
        if let Some(ref mut m) = self.mapper {
            let result = m.ppu_map_read(address);
            if result.read_from_cart_ram {
                *data = self.chr_memory[result.mapped_address as usize];
                return true;
            } else if result.read_from_mapper_ram {
                *data = result.data;
                return true;
            }
        }

        false
    }

    /// Write to the PPU Bus
    pub fn ppu_write(&mut self, address: u16, data: u8) -> bool {
        let mut mapped_address: u32 = 0;
        if let Some(ref mut m) = self.mapper {
            if m.ppu_map_write(address, &mut mapped_address, data) {
                self.chr_memory[mapped_address as usize] = data;
                return true;
            }
        }

        false
    }

    pub fn get_mirror(&self) -> mirror::Mirror {
        match self.mapper {
            Some(ref mapper) => {
                let mirror = mapper.get_mirror();
                match mirror {
                    mirror::Mirror::Hardware => self.mirror,
                    _ => mirror
                }
            },
            None => self.mirror
        }
    }

    pub fn save_data(&mut self) {
        if let Some(ref mut m) = self.mapper {
            m.save_battery_backed_ram(&self.file_path);
        }
    }

    fn get_mapper(mapper_id: u8, header: &cartridge_header::CartridgeHeader, file_name: &str) -> Option<Box<dyn mappers::mapper::Mapper>> {
        let prg_banks = header.prg_rom_chunks;
        let chr_banks = header.chr_rom_chunks;
        let has_battery_backed_ram = (header.mapper_1 >> 1) & 1 != 0;
        let mut mapper: Option<Box<dyn mappers::mapper::Mapper>> =  match mapper_id {
            0 => Some(Box::new(mappers::mapper000::Mapper000::new(prg_banks, chr_banks, has_battery_backed_ram))),
            1 => Some(Box::new(mappers::mapper001::Mapper001::new(prg_banks, chr_banks, has_battery_backed_ram))),
            2 => Some(Box::new(mappers::mapper002::Mapper002::new(prg_banks, chr_banks, has_battery_backed_ram))),
            3 => Some(Box::new(mappers::mapper003::Mapper003::new(prg_banks, chr_banks, has_battery_backed_ram))),
            4 => Some(Box::new(mappers::mapper004::Mapper004::new(prg_banks, chr_banks, has_battery_backed_ram))),
           66 => Some(Box::new(mappers::mapper066::Mapper066::new(prg_banks, chr_banks, has_battery_backed_ram))),
            _ => None
        };

        if let Some(ref mut m) = mapper {
            mappers::battery_backed_ram::load_battery_backed_ram(m, file_name);
        }

        mapper
    }
}