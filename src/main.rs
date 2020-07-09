use ggez::*;
use rodio::Sink;
use rodio::Source;
use std::time;

mod memory;
mod bus;
mod cpu;
mod ppu;
mod cartridge;
mod memory_sizes;
mod display;
mod frame;
mod addresses;
mod controller;

fn main() {
    // run_game();
    play_sound();
}

fn run_game() {
    let mut bus = bus::Bus::new();
    let cartridge = cartridge::cartridge::Cartridge::new(r"C:\Users\Christopher\Desktop\Files\NES\ROMS\Super Mario Bros. (World).nes");
    bus.load_cartridge(cartridge);
    
    bus.reset();

    let mut configuration = conf::Conf::new();
    configuration.window_setup = conf::WindowSetup::default().title("NES");
    configuration.window_mode = conf::WindowMode::default().dimensions(display::WINDOW_WIDTH as f32, display::WINDOW_HEIGHT as f32);

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("NES", "caklimas@gmail.com") 
        .conf(configuration)
        .build()
        .expect("Error building context");

    event::run(ctx, event_loop, &mut bus).expect("Error running loop");
}

fn play_sound() {
    let device = rodio::default_output_device().expect("Error loading audio device");
    let sink = Sink::new(&device);
    let source1 = rodio::source::SineWave::new(300).take_duration(time::Duration::from_secs(5));
    let source2 = rodio::source::SineWave::new(400).take_duration(time::Duration::from_secs(2));
    sink.append(source1);
    sink.append(source2);
    sink.play();
    loop {

    }
}