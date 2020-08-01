use crate::cartridge::mirror::Mirror;

bitfield! {
    pub struct ControlRegister(u8);
    impl Debug;

    pub mirror, set_mirror: 1, 0;
    pub prg_mode, set_prg_mode: 3, 2;
    pub chr_mode, set_chr_mode: 4;
    pub get, set: 4, 0; 
}

impl ControlRegister {
    pub fn get_mirror(&self) -> Mirror {
        match self.mirror() {
            0 => Mirror::OneScreenLow,
            1 => Mirror::OneScreenHigh,
            2 => Mirror::Vertical,
            3 => Mirror::Horizontal,
            _ => panic!("Invalid mirror mode")
        }
    }

    pub fn get_prg_mode(&self) -> PrgBankMode {
        match self.prg_mode() {
            0..=1 => PrgBankMode::Switch32KB,
            2 => PrgBankMode::FixFirst,
            3 => PrgBankMode::FixLast,
            _ => panic!("Invalid PRG Bank mode")
        }
    }

    pub fn get_chr_mode(&self) -> ChrBankMode {
        match self.chr_mode() {
            false => ChrBankMode::Switch8KB,
            true => ChrBankMode::SwitchTwo4KB
        }
    }

    pub fn write(&mut self, data: u8) {
        self.set_mirror(data & 0b11);
        self.set_prg_mode((data & 0b1100) >> 2);
        self.set_chr_mode(((data & 0b10000) >> 4) != 0);
    }
}

#[derive(Debug)]
pub enum PrgBankMode {
    Switch32KB, // switch 32 KB at $8000, ignoring low bit of bank number
    FixFirst,   // fix first bank at $8000 and switch 16 KB bank at $C000
    FixLast     // fix last bank at $C000 and switch 16 KB bank at $8000
}

#[derive(Debug)]
pub enum ChrBankMode {
    Switch8KB,   // switch 8 KB at a time
    SwitchTwo4KB // switch two separate 4 KB banks
}