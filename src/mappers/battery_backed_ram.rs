use std::fs;
use std::path::Path;
use super::mapper::Mapper;

pub fn save_battery_backed_ram(file_path: &str, data: &Vec<u8>) {
    println!("Saving {} bytes of data", data.len());
    let save_data = get_save_data_path(file_path);
    fs::write(save_data, data).expect("Error writing save data to path");
}

pub fn load_battery_backed_ram(mapper: &mut Box<dyn Mapper>, file_path: &str) {
    let save_data = get_save_data_path(file_path);
    let save_data_path = Path::new(&save_data);
    if !save_data_path.exists() {
        return;
    }

    let bytes = fs::read(save_data_path).expect("Error reading save data");
    mapper.load_battery_backed_ram(bytes);
}

fn get_save_data_path(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut save_data = path.parent().expect("ROM must be in a directory").to_str().expect("Error converting directory to string").to_owned();
    let file_name = path.file_name().expect("Error getting name of file").to_str().expect("Error file name to string");
    save_data.push_str(r"\");
    save_data.push_str(file_name);
    save_data.push_str(".sav");

    save_data
}