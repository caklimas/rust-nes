use std::fmt::{Debug, Formatter, Result};
use crate::display;
use super::colors::Color;

const BYTES_PER_COLUMN: usize = display::PIXEL_SIZE * display::BYTES_PER_COLOR;
const BYTES_PER_ROW: usize = BYTES_PER_COLUMN * display::SCREEN_WIDTH;
const BYTE_WIDTH: usize = BYTES_PER_COLUMN * display::SCREEN_WIDTH;
const BYTE_HEIGHT: usize = display::SCREEN_HEIGHT * display::PIXEL_SIZE;

pub struct Frame {
    pixels: Vec<u8>
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            pixels: vec![0; BYTE_WIDTH * BYTE_HEIGHT]
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= display::SCREEN_WIDTH || y >= display::SCREEN_HEIGHT {
            return;
        }

        let (red, green, blue) = color;
        let y_offset = y * BYTES_PER_ROW * display::PIXEL_SIZE;
        for sdl_row_num in 0..display::PIXEL_SIZE {
            let row_offset = y_offset + (sdl_row_num * BYTES_PER_ROW);
            let x_offset = x * BYTES_PER_COLUMN;
            for sdl_col_num in 0..display::PIXEL_SIZE {
                let col_offset = x_offset + (sdl_col_num * 3);
                let offset = row_offset + col_offset;
                self.pixels[offset] = red;
                self.pixels[offset + 1] = green;
                self.pixels[offset + 2] = blue;
            }
        }
    }

    pub fn get_pixels(&self) -> &[u8] {
        &self.pixels
    }
}

impl Default for Frame {
    fn default() -> Self {
        Frame::new()
    }
}

impl Debug for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Ppu2C02")
         .field("pixel_length", &self.pixels.len())
         .finish()
    }
}