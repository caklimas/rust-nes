#[macro_use]
extern crate bitfield;

#[macro_use]
extern crate serde_big_array;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode};
use sdl2::messagebox::*;
use sdl2::pixels::PixelFormatEnum;
use std::env;
use std::path::Path;
use std::string::String;
use std::sync::{Arc, Mutex};

mod addresses;
mod audio;
mod bus;
mod cartridge;
mod controller;
mod cpu;
mod display;
mod instant;
mod mappers;
mod memory_sizes;
mod nes;
mod ppu;
mod save_state;

use audio::device::AudioDevice;

fn main() {
    let buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
    let sdl_buffer = Arc::clone(&buffer);
    let sdl_context = sdl2::init().expect("Error initializing sdl");
    let audio_device = AudioDevice::new(&sdl_context, sdl_buffer);
    run_game(&sdl_context, &audio_device, buffer);
}

fn run_game(sdl_context: &Sdl, audio_device: &sdl2::audio::AudioDevice<AudioDevice>, buffer: Arc<Mutex<Vec<f32>>>) {
    let (mut canvas, texture_creator) = display::initialize_window(sdl_context);
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24,
        (display::SCREEN_WIDTH * display::PIXEL_SIZE) as u32,
        (display::SCREEN_HEIGHT * display::PIXEL_SIZE) as u32
    ).expect("Error creating texture streaming");

    let mut audio_started = false;
    let args: Vec<String> = env::args().collect();
    let mut nes = get_nes(&args[1], buffer);

    let mut event_pump = sdl_context.event_pump().expect("Error loading event pump");
    'running: loop {
        let frame_complete = nes.clock(&mut texture, &mut canvas, &event_pump);
        if frame_complete {
            if !audio_started {
                audio_started = true;
                audio_device.resume();
            }

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        if let Some(ref mut c) = nes.bus().cartridge {
                            c.borrow_mut().save_data();
                        }

                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::F7), .. } => {
                        let file_path: Option<String> = match nes.bus().cartridge {
                            Some(ref c) => Some(String::from(&c.borrow().file_path)),
                            None => None
                        };

                        if let Some(ref f) = file_path {
                            save_state::quick_save(&mut nes, &f);
                            show_simple_message_box(
                                MessageBoxFlag::INFORMATION,
                                "Quick save",
                                "Data has been saved",
                                canvas.window()).expect("Error showing simple message");
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn get_nes(file_path: &str, buffer: Arc<Mutex<Vec<f32>>>) -> nes::Nes {
    let path = Path::new(file_path);
    let os_extension = path.extension().expect("Error getting file extension");
    let extension = os_extension.to_str().expect("Error converting to string");
    
    match extension {
        "nes" => {
            let mut nes = nes::Nes::new(buffer);
            let cartridge = cartridge::Cartridge::new(file_path);
            nes.bus().load_cartridge(cartridge);
            nes.reset();
            nes
        },
        "qks" => {
            let nes = save_state::quick_load(file_path, buffer);
            nes
        },
        _ => panic!("Unrecognized file extension")
    }
}