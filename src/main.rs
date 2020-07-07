use ggez::*;

mod memory;
mod bus;
mod cpu;
mod opcodes;
mod address_modes;
mod opcode_table;
mod ppu;
mod cartridge;
mod mappers;
mod memory_sizes;
mod display;
mod frame;
mod addresses;

fn main() {
    let mut bus = bus::Bus::new();
    let cartridge = cartridge::Cartridge::new(r".\src\test_roms\Ice Climber (USA, Europe).nes");
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