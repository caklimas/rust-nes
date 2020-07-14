use ggez::*;
use ggez::event::{
    KeyCode,
    KeyMods
};

use crate::nes;

const PIXEL_SIZE: i32 = 3;
pub const SCREEN_WIDTH: u16 = 256;
pub const SCREEN_HEIGHT: u16 = 240;
pub const WINDOW_WIDTH: f32 = SCREEN_WIDTH as f32 * PIXEL_SIZE as f32;
pub const WINDOW_HEIGHT: f32 = SCREEN_HEIGHT as f32 * PIXEL_SIZE as f32;

impl ggez::event::EventHandler for nes::Nes {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.can_draw {
            return Ok(());
        }

        loop {
            self.clock();
            if self.ppu().frame_complete {
                break;
            }
        }

        loop {
            self.clock();
            if self.cpu.is_complete() {
                break;
            }
        }

        self.ppu().frame_complete = false;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.can_draw {
            return Ok(());
        }

        graphics::clear(ctx, graphics::BLACK);
        self.draw_frame(ctx);
        // let mut mesh_builder = graphics::MeshBuilder::new();
        // let mut draw = false;
        // let oam = &self.ppu.borrow_mut().oam;
        // for i in 0..64 {
        //     let color = graphics::Color::from_rgb(250, 243, 17);
        //     let x = oam[(i * sprites::OAM_ENTRY_SIZE) + 3];
        //     let y = oam[(i * sprites::OAM_ENTRY_SIZE) + 0];
        //     let rect = graphics::Rect::new_i32(
        //         x as i32 * PIXEL_SIZE, 
        //         y as i32 * PIXEL_SIZE, 
        //         PIXEL_SIZE, 
        //         PIXEL_SIZE
        //     );
        //     mesh_builder.rectangle(graphics::DrawMode::fill(), rect, color);
    
        //     if y >= 24 && y <= 31 {
        //         let sdjlkfhsd = 4;
        //         // println!("({}, {})", x, y)
        //     }
        // }

        // let mesh = mesh_builder.build(ctx).expect("Error building the mesh");
        // graphics::draw(ctx, &mesh, (nalgebra::Point2::new(0.0, 0.0),)).expect("Error drawing");
        graphics::present(ctx).expect("Error presenting graphics");

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::L => {
                println!("Writing");
                println!("Done writing");
            },
            KeyCode::C => {
                self.clock();
                while self.cpu.cycles != 0 {
                    self.clock();
                }
            },
            KeyCode::D => {
                self.can_draw = !self.can_draw;
                if self.can_draw {
                    println!("Start");
                } else {
                    println!("Stop");
                }
            },
            KeyCode::Right => {
                self.bus().controllers[0].buttons[0] = true;
            },
            KeyCode::Left => {
                self.bus().controllers[0].buttons[1] = true;
            },
            KeyCode::Down => {
                self.bus().controllers[0].buttons[2] = true;
            },
            KeyCode::Up => {
                self.bus().controllers[0].buttons[3] = true;
            },
            KeyCode::Return => {
                self.bus().controllers[0].buttons[4] = true;
            },
            KeyCode::RShift => {
                self.bus().controllers[0].buttons[5] = true;
            },
            KeyCode::Z => {
                self.bus().controllers[0].buttons[6] = true;
            },
            KeyCode::X => {
                self.bus().controllers[0].buttons[7] = true;
            }
            _ => ()
        };
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Right => {
                self.bus().controllers[0].buttons[0] = false;
            },
            KeyCode::Left => {
                self.bus().controllers[0].buttons[1] = false;
            },
            KeyCode::Down => {
                self.bus().controllers[0].buttons[2] = false;
            },
            KeyCode::Up => {
                self.bus().controllers[0].buttons[3] = false;
            },
            KeyCode::Return => {
                self.bus().controllers[0].buttons[4] = false;
            },
            KeyCode::RShift => {
                self.bus().controllers[0].buttons[5] = false;
            },
            KeyCode::Z => {
                self.bus().controllers[0].buttons[6] = false;
            },
            KeyCode::X => {
                self.bus().controllers[0].buttons[7] = false;
            }
            _ => ()
        };
    }
}

impl nes::Nes {
    pub fn draw_frame(&mut self, ctx: &mut Context) {
        let mut mesh_builder = graphics::MeshBuilder::new();
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                let color = self.ppu().frame.get_pixel(i as i32, j as i32);
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