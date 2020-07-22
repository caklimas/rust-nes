#[macro_use]
extern crate bitfield;

use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod nes;
mod bus;
mod cpu;
mod ppu;
mod cartridge;
mod mappers;
mod memory_sizes;
mod display;
mod addresses;
mod controller;
mod audio;

fn main() {
    let buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
    let sdl_buffer = Arc::clone(&buffer);
    let sdl_context = sdl2::init().expect("Error initializing sdl");
    
    // std::thread::spawn(move || {
        
    //     let audio_device = audio::device::AudioDevice::new(&sdl_context, sdl_buffer);
    //     audio_device.resume();

    //     loop {
            
    //     }
    // });
    run_game(&sdl_context, buffer);
}

fn run_game(sdl_context: &Sdl, buffer: Arc<Mutex<Vec<f32>>>) {
    let (mut canvas, texture_creator) = display::initialize_window(sdl_context);
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24,
        (display::SCREEN_WIDTH * display::PIXEL_SIZE) as u32,
        (display::SCREEN_HEIGHT * display::PIXEL_SIZE) as u32
    ).expect("Error creating texture streaming");

    let args: Vec<String> = env::args().collect();
    let mut nes = nes::Nes::new(buffer);
    let cartridge = cartridge::cartridge::Cartridge::new(&args[1]);
    nes.bus().load_cartridge(cartridge);
    
    nes.reset();

    let mut event_pump = sdl_context.event_pump().expect("Error loading event pump");
    'running: loop {
        let frame_complete = nes.clock(&mut texture, &mut canvas);
        if frame_complete {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
        }
    }
}