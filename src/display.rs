use ggez::*;
use ggez::event::{
    KeyCode,
    KeyMods
};
use rand::Rng;

use crate::bus;

const PIXEL_SIZE: i32 = 4;
pub const SCREEN_WIDTH: u16 = 256;
pub const SCREEN_HEIGHT: u16 = 240;
pub const WINDOW_WIDTH: f32 = SCREEN_WIDTH as f32 * PIXEL_SIZE as f32;
pub const WINDOW_HEIGHT: f32 = SCREEN_HEIGHT as f32 * PIXEL_SIZE as f32;

impl ggez::event::EventHandler for bus::Bus {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.can_draw {
            return Ok(());
        }

        graphics::clear(ctx, graphics::BLACK);

        let colors = self.ppu.borrow_mut().colors;
        let mut rand = rand::thread_rng();
        let mut mesh_builder = graphics::MeshBuilder::new();

        for row in 0..SCREEN_HEIGHT {
            for column in 0..SCREEN_WIDTH {
                let random = rand.gen_range(0, colors.len());
                let color = colors[random];

                let rect = graphics::Rect::new_i32(
                    column as i32 * PIXEL_SIZE, 
                    row as i32 * PIXEL_SIZE, 
                    PIXEL_SIZE, 
                    PIXEL_SIZE
                );
    
                mesh_builder.rectangle(graphics::DrawMode::fill(), rect, color);
            }
        }
        
        let mesh = mesh_builder.build(ctx).expect("Error building the mesh");
        graphics::draw(ctx, &mesh, (nalgebra::Point2::new(0.0, 0.0),)).expect("Error drawing");
        graphics::present(ctx).expect("Error presenting graphics");
        self.can_draw = false;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::C => {
                self.clock();
            },
            KeyCode::D => {
                self.can_draw = true;
            },
            _ => ()
        }
    }
}