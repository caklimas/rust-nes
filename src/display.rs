use mint::Point2;
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
        if self.emulation {
            while !self.ppu.borrow().frame_complete {
                self.clock();
            }

            self.ppu.borrow_mut().frame_complete = false;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // if !self.can_draw {
        //     return Ok(());
        // }

        graphics::clear(ctx, graphics::BLACK);

        self.draw_pattern_tables(ctx);
        graphics::present(ctx).expect("Error presenting graphics");
        self.can_draw = false;

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
                self.can_draw = true;
            },
            KeyCode::Space => {
              self.emulation = !self.emulation;
              if !self.emulation {
                  println!("Stop");
              } else {
                println!("Start");
              }
            },
            _ => ()
        };
    }
}

impl bus::Bus {
    pub fn draw_random_colors(&mut self, ctx: &mut Context) {
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
    }

    pub fn draw_table_ids(&mut self, ctx: &mut Context) {
        for height in 0..30 {
            for width in 0..32 {
                let text = graphics::Text::new(graphics::TextFragment {
                    // `TextFragment` stores a string, and optional parameters which will override those
                    // of `Text` itself. This allows inlining differently formatted lines, words,
                    // or even individual letters, into the same block of text.
                    text: self.ppu.borrow_mut().name_table[0][height * 32 + width].to_string(),
                    color: Some(graphics::WHITE),
                    // `Font` is a handle to a loaded TTF, stored inside the `Context`.
                    // `Font::default()` always exists and maps to DejaVuSerif.
                    font: Some(graphics::Font::default()),
                    scale: Some(graphics::Scale::uniform(10.0)),
                    // This doesn't do anything at this point; can be used to omit fields in declarations.
                    ..Default::default()
                });

                graphics::queue_text(ctx, &text, Point2 { x: (width * 32) as f32, y: (height * 32) as f32 }, Some(graphics::WHITE));
            }
        }

        
        graphics::draw_queued_text(ctx, graphics::DrawParam::default(), None, graphics::FilterMode::Linear).expect("Draw text failed");
    }

    pub fn draw_pattern_tables(&mut self, ctx: &mut Context) {
        let table = self.ppu.borrow_mut().get_pattern_table(0, 1);
        let mut mesh_builder = graphics::MeshBuilder::new();

        for row in 0..128 {
            for column in 0..128 {
                let rect = graphics::Rect::new_i32(
                    column as i32 * PIXEL_SIZE, 
                    row as i32 * PIXEL_SIZE, 
                    PIXEL_SIZE, 
                    PIXEL_SIZE
                );
    
                mesh_builder.rectangle(graphics::DrawMode::fill(), rect, table[row][column]);
            }
        }
        
        let mesh = mesh_builder.build(ctx).expect("Error building the mesh");
        graphics::draw(ctx, &mesh, (nalgebra::Point2::new(0.0, 0.0),)).expect("Error drawing");
    }
}