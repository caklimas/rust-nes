const RAM_SIZE: usize = 2048;
const ram: [u8; RAM_SIZE] = [0; RAM_SIZE];

pub fn read(address: u16, read_only: bool) -> u8 {
    ram[(address) as usize]
}

pub fn write(address: u16, data: u8) {
    ram[(address) as usize] = data;
}