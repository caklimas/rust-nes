use ggez::*;

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
    let mut bus = bus::Bus::new();
    let cartridge = cartridge::cartridge::Cartridge::new(r"C:\Users\Christopher\Desktop\Files\ROMS\blargg_ppu_tests_2005.09.15b\palette_ram.nes");
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