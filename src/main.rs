#[macro_use]
extern crate bitfield;

use ggez::*;
use std::env;
use std::sync::{Arc, Mutex};

mod nes;
mod bus;
mod cpu;
mod ppu;
mod cartridge;
mod mappers;
mod memory_sizes;
mod display;
mod frame;
mod addresses;
mod controller;
mod audio;

fn main() {
    let buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
    let sdl_buffer = Arc::clone(&buffer);
    let test_buffer = Arc::clone(&buffer);
    std::thread::spawn(move || {
        let sdl_context = sdl2::init().unwrap();
        let audio_device = audio::device::AudioDevice::new(&sdl_context, sdl_buffer);
        audio_device.resume();

        loop {
            
        }
    });
    // std::thread::spawn(move || {
    //     let mut temp_buffer = Vec::<f32>::new();
    //     let mut sine = audio::sine::SineWave::new(440.0, 0.5);
    //     loop {
    //         for _i in 0..(2048 * 100) {
    //             temp_buffer.push(sine.get_next());
    //         }

    //         {
    //             let mut lock = test_buffer.lock().expect("Error locking the buffer");
    //             lock.append(&mut temp_buffer);
    //         }

    //         std::thread::sleep(std::time::Duration::from_millis(2000));
    //     }
    // });
    run_game(buffer);
}

fn run_game(buffer: Arc<Mutex<Vec<f32>>>) {
    let args: Vec<String> = env::args().collect();
    let mut nes = nes::Nes::new(buffer);
    let cartridge = cartridge::cartridge::Cartridge::new(&args[1]);
    nes.bus().load_cartridge(cartridge);
    
    nes.reset();

    let mut configuration = conf::Conf::new();
    configuration.window_setup = conf::WindowSetup::default().title("NES");
    configuration.window_mode = conf::WindowMode::default().dimensions(display::WINDOW_WIDTH as f32, display::WINDOW_HEIGHT as f32);

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("NES", "caklimas@gmail.com") 
        .conf(configuration)
        .build()
        .expect("Error building context");

    event::run(ctx, event_loop, &mut nes).expect("Error running loop");
}

fn render_sdl2() {
    
}