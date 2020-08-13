use serde::{Serialize, Deserialize};
use crate::cartridge::mirror::Mirror;
use super::mapper001::*;
use super::mapper004::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum MapperSaveData {
    None,
    Mapper000(Mapper000SaveData),
    Mapper001(Mapper001SaveData),
    Mapper002(Mapper002SaveData),
    Mapper003(Mapper003SaveData),
    Mapper004(Mapper004SaveData),
    Mapper066(Mapper066SaveData)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mapper000SaveData {
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub battery_backed_ram: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mapper001SaveData {
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub battery_backed_ram: bool,
    pub chr_bank: chr_bank::ChrBank,
    pub control_register: control_register::ControlRegister,
    pub prg_bank: prg_bank::PrgBank,
    pub ram: Vec<u8>,
    pub shift_register: shift_register::ShiftRegister
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mapper002SaveData {
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub prg_bank_low: u8,
    pub prg_bank_high: u8,
    pub battery_backed_ram: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mapper003SaveData {
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub battery_backed_ram: bool,
    pub chr_bank: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mapper004SaveData {
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub interrupt_request: interrupt_request::InterruptRequest,
    pub bank_select: bank_select::BankSelect,
    pub battery_backed_ram: bool,
    pub mirror: Mirror,
    pub prg_ram_protect: prg_ram_protect::PrgRamProtect,
    pub ram: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mapper066SaveData {
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub battery_backed_ram: bool,
    pub chr_bank: u8,
    pub prg_bank: u8
}