#[macro_use]
extern crate bitfield;

use std::env;
use std::sync::{Arc, Mutex};
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::PixelFormatEnum;

mod addresses;
mod audio;
mod bus;
mod cartridge;
mod controller;
mod cpu;
mod display;
mod mappers;
mod memory_sizes;
mod nes;
mod ppu;

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
    let mut nes = nes::Nes::new(buffer);
    let cartridge = cartridge::cartridge::Cartridge::new(&args[1]);
    //let cartridge = cartridge::cartridge::Cartridge::new(r"C:\Users\Christopher\Desktop\Files\NES\ROMS\Castlevania.nes");
    nes.bus().load_cartridge(cartridge);
    
    nes.reset();

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
                        match nes.bus().cartridge {
                            Some(ref mut c) => c.borrow_mut().save_data(),
                            None => ()
                        }
                        break 'running
                    },
                    _ => {}
                }
            }
        }
    }
}