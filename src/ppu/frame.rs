use super::Color;

pub const FRAME_WIDTH: usize = 256;
pub const FRAME_HEIGHT: usize = 240;

pub struct Frame {
    pixels: [[Color; FRAME_WIDTH]; FRAME_HEIGHT]
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            pixels: [[(0, 0, 0); FRAME_WIDTH]; FRAME_HEIGHT]
        }
    }

    pub fn get_pixel(&mut self, x: i32, y: i32) -> Color {
        if (x < 0 || x > (FRAME_WIDTH - 1) as i32) || (y < 0 || y > (FRAME_HEIGHT - 1) as i32) {
            return (0, 0, 0);
        }

        self.pixels[y as usize][x as usize]
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < (FRAME_WIDTH as i32) && y >= 0 && y < (FRAME_HEIGHT as i32) {
            self.pixels[y as usize][x as usize] = color;
        }
    }
}