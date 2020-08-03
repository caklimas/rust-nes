pub struct MapperReadResult {
    pub data: u8,
    pub mapped_address: u32,
    pub read_from_cart_ram: bool,
    pub read_from_mapper_ram: bool
}

impl MapperReadResult {
    pub fn from_cart_ram(mapped_address: u32) -> Self {
        MapperReadResult {
            data: 0,
            mapped_address,
            read_from_cart_ram: true,
            read_from_mapper_ram: false
        }
    }

    pub fn from_mapper_ram(data: u8) -> Self {
        MapperReadResult {
            data,
            mapped_address: 0,
            read_from_cart_ram: false,
            read_from_mapper_ram: true
        }
    }

    pub fn none() -> Self {
        MapperReadResult {
            data: 0,
            mapped_address: 0,
            read_from_cart_ram: false,
            read_from_mapper_ram: false
        }
    }
}

pub struct MapperWriteResult {
    pub handled: bool,
    pub mapped_address: u32,
    pub write_to_cart_ram: bool
}

impl MapperWriteResult {
    pub fn handled() -> Self {
        MapperWriteResult {
            handled: true,
            mapped_address: 0,
            write_to_cart_ram: false
        }
    }

    pub fn write_to_cart_ram(mapped_address: u32) -> Self {
        MapperWriteResult {
            handled: true,
            mapped_address,
            write_to_cart_ram: true
        }
    }

    pub fn with_mapped_address(mapped_address: u32) -> Self {
        MapperWriteResult {
            handled: true,
            mapped_address,
            write_to_cart_ram: false
        }
    }

    pub fn none() -> Self {
        MapperWriteResult {
            handled: false,
            mapped_address: 0,
            write_to_cart_ram: false
        }
    }
}