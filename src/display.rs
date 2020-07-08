use ggez::*;
use ggez::event::{
    KeyCode,
    KeyMods
};
use std::time::{Duration, Instant};

use crate::bus;

const PIXEL_SIZE: i32 = 3;
pub const SCREEN_WIDTH: u16 = 256;
pub const SCREEN_HEIGHT: u16 = 240;
pub const WINDOW_WIDTH: f32 = SCREEN_WIDTH as f32 * PIXEL_SIZE as f32;
pub const WINDOW_HEIGHT: f32 = SCREEN_HEIGHT as f32 * PIXEL_SIZE as f32;

impl ggez::event::EventHandler for bus::Bus {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        loop {
            self.clock();
            if self.ppu.borrow().frame_complete {
                break;
            }
        }

        loop {
            self.clock();
            if self.cpu.is_complete() {
                break;
            }
        }

        self.ppu.borrow_mut().frame_complete = false;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.draw_frame(ctx);
        graphics::present(ctx).expect("Error presenting graphics");

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::C => {
                self.clock();
                while self.cpu.cycles != 0 {
                    self.clock();
                }
            },
            KeyCode::D => {
                self.can_draw = !self.can_draw;
            },
            KeyCode::Right => {
                self.memory.borrow_mut().controllers[0].buttons[0] = true;
            },
            KeyCode::Left => {
                self.memory.borrow_mut().controllers[0].buttons[1] = true;
            },
            KeyCode::Down => {
                self.memory.borrow_mut().controllers[0].buttons[2] = true;
            },
            KeyCode::Up => {
                self.memory.borrow_mut().controllers[0].buttons[3] = true;
            },
            KeyCode::Return => {
                self.memory.borrow_mut().controllers[0].buttons[4] = true;
            },
            KeyCode::RShift => {
                self.memory.borrow_mut().controllers[0].buttons[5] = true;
            },
            KeyCode::Z => {
                self.memory.borrow_mut().controllers[0].buttons[6] = true;
            },
            KeyCode::X => {
                self.memory.borrow_mut().controllers[0].buttons[7] = true;
            }
            _ => ()
        };
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Right => {
                self.memory.borrow_mut().controllers[0].buttons[0] = false;
            },
            KeyCode::Left => {
                self.memory.borrow_mut().controllers[0].buttons[1] = false;
            },
            KeyCode::Down => {
                self.memory.borrow_mut().controllers[0].buttons[2] = false;
            },
            KeyCode::Up => {
                self.memory.borrow_mut().controllers[0].buttons[3] = false;
            },
            KeyCode::Return => {
                self.memory.borrow_mut().controllers[0].buttons[4] = false;
            },
            KeyCode::RShift => {
                self.memory.borrow_mut().controllers[0].buttons[5] = false;
            },
            KeyCode::Z => {
                self.memory.borrow_mut().controllers[0].buttons[6] = false;
            },
            KeyCode::X => {
                self.memory.borrow_mut().controllers[0].buttons[7] = false;
            }
            _ => ()
        };
    }
}

impl bus::Bus {
    pub fn draw_frame(&mut self, ctx: &mut Context) {
        let mut mesh_builder = graphics::MeshBuilder::new();
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                let color = self.ppu.borrow_mut().frame.get_pixel(i as i32, j as i32);
                let rect = graphics::Rect::new_i32(
                    i as i32 * PIXEL_SIZE, 
                    j as i32 * PIXEL_SIZE, 
                    PIXEL_SIZE, 
                    PIXEL_SIZE
                );
    
                mesh_builder.rectangle(graphics::DrawMode::fill(), rect, color);
            }
        }

        let mesh = mesh_builder.build(ctx).expect("Error building the mesh");
        graphics::draw(ctx, &mesh, (nalgebra::Point2::new(0.0, 0.0),)).expect("Error drawing");
    }
}