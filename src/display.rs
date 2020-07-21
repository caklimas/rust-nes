use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub const PIXEL_SIZE: usize = 3;
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;
pub const BYTES_PER_COLOR: usize = 3;

pub fn initialize_window(context: &Sdl) -> (Canvas<Window>, TextureCreator<WindowContext>) {
    let video_subsystem = context.video().expect("Error getting video subsystem");
    let window = video_subsystem.window("NES", (SCREEN_WIDTH * PIXEL_SIZE) as u32, (SCREEN_HEIGHT * PIXEL_SIZE) as u32)
        .position_centered()
        .opengl()
        .build()
        .expect("Error setting up window");

    let mut canvas = window.into_canvas().build().expect("Error building canvas");
    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    (canvas, texture_creator)
}

pub fn draw_frame(texture: &mut Texture, canvas: &mut Canvas<Window>, buffer: &[u8]) {
    texture.update(None, buffer, SCREEN_WIDTH * BYTES_PER_COLOR * PIXEL_SIZE).expect("Error updating texture");
    canvas.copy(texture, None, None).expect("Error copying to canvas");
    canvas.present();
}