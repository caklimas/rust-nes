use ggez::graphics;
use ggez::graphics::Color;

pub const FRAME_WIDTH: u16 = 256;
pub const FRAME_HEIGHT: u16 = 240;

pub struct Frame {
    pixels: Vec<[Color; FRAME_WIDTH as usize]>
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            pixels: Frame::initialize_frame()
        }
    }

    pub fn get_pixel(&mut self, x: i32, y: i32) -> Color {
        if (x < 0 || x > (FRAME_WIDTH - 1) as i32) || (y < 0 || y > (FRAME_HEIGHT - 1) as i32) {
            return graphics::BLACK;
        }

        self.pixels[y as usize][x as usize]
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < (FRAME_WIDTH as i32) && y >= 0 && y < (FRAME_HEIGHT as i32) {
            self.pixels[y as usize][x as usize] = color;
        }
    }

    fn initialize_frame() -> Vec<[Color; 256]> {
        let mut frame: Vec<[Color; FRAME_WIDTH as usize]> = Vec::new();
        for _y in 0..FRAME_HEIGHT {
            frame.push([graphics::BLACK; FRAME_WIDTH as usize]);
        }
    
        frame
    }
}