pub mod chr_bank;
pub mod control_register;
pub mod mapper001;
pub mod prg_bank;
pub mod shift_register;

const CHR_ROM_FIRST_BANK_LOWER: u16 = 0x0000;
const CHR_ROM_FIRST_BANK_UPPER: u16 = 0x0FFF;
const CHR_ROM_LAST_BANK_LOWER: u16 = 0x1000;
const CHR_ROM_LAST_BANK_UPPER: u16 = 0x1FFF;
const OPTIONAL_RAM_ADDRESS_LOWER: u16 = 0x6000;
const OPTIONAL_RAM_ADDRESS_UPPER: u16 = 0x7FFF;
const PRG_ROM_FIRST_BANK_LOWER: u16 = 0x8000;
const PRG_ROM_FIRST_BANK_UPPER: u16 = 0xBFFF;
const PRG_ROM_LAST_BANK_LOWER: u16 = 0xC000;
const PRG_ROM_LAST_BANK_UPPER: u16 = 0xFFFF;
const RAM_ADDRESS_MASK: u16 = 0x1FFF;