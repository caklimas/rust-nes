const RAM_SIZE: usize = 2048;
const RAM: [u8; RAM_SIZE] = [0; RAM_SIZE];

pub fn read(address: u16, read_only: bool) -> u8 {
    RAM[(address) as usize]
}

pub fn write(address: u16, data: u8) {
    RAM[(address) as usize] = data;
}

pub fn reset() {
    for i in 0..RAM.len() {
        RAM[i] = 0
    }
}