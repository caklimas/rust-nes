use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::str;
use crate::nes::Nes;

pub fn quick_save(nes: &mut Nes, file_path: &str) {
    if let Some(ref mut c) = nes.bus().cartridge {
        c.borrow_mut().save_mapper();
    }

    let serialized = serde_json::to_string(nes).expect("Error serializing the NES");
    let save_data_path = get_save_data_path(file_path);
    fs::write(save_data_path, serialized).expect("Error writing save data to path");
}

pub fn quick_load(file_path: &str, buffer: Arc<Mutex<Vec<f32>>>) -> Nes {
    let bytes = fs::read(file_path).expect("Error reading save data");
    let data = str::from_utf8(&bytes).expect("Error converting quicksave bytes to string");
    let mut nes: Nes = serde_json::from_str(&data).expect("Error loading save data");
    nes.load_buffer(buffer);

    if let Some(ref mut c) = nes.cpu.bus.cartridge {
        c.borrow_mut().load_mapper();
    }

    if let Some(ref mut c) = nes.cpu.bus.ppu.cartridge {
        c.borrow_mut().load_mapper();
    }

    nes
}

fn get_save_data_path(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut save_data = path.parent().expect("ROM must be in a directory").to_str().expect("Error converting directory to string").to_owned();
    let file_name = path.file_name().expect("Error getting name of file").to_str().expect("Error file name to string");
    save_data.push_str(r"\");
    save_data.push_str(file_name);
    save_data.push_str(".qks");

    save_data
}