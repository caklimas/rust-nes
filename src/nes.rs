use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use sdl2::keyboard::{Scancode};
use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window};

use crate::cpu;
use crate::bus;
use crate::ppu;
use crate::audio;
use crate::display;

pub struct Nes {
    pub cpu: cpu::Cpu6502,
    fps_limiter: ppu::fps_limiter::FpsLimiter,
    timer: Instant,
    system_clock_counter: u32,
    dma_dummy: bool,
    buffer: Arc<Mutex<Vec<f32>>>
}

impl Nes {
    pub fn new(buffer: Arc<Mutex<Vec<f32>>>) -> Self {
        Nes {
            cpu: cpu::Cpu6502::new(),
            fps_limiter: ppu::fps_limiter::FpsLimiter::new(60),
            timer: Instant::now(),
            system_clock_counter: 0,
            dma_dummy: false,
            buffer
        }
    }

    pub fn clock(&mut self, texture: &mut Texture, canvas: &mut Canvas<Window>, event_pump: &sdl2::EventPump) -> bool {
        let frame_complete = self.ppu().clock();

        // The CPU runs 3 times slower than the PPU
        if self.system_clock_counter % 3 == 0 {
            // If DMA transer is happening, then the cpu is suspended
            if self.bus().dma_transfer {
                self.dma_transfer();
            } else {
                self.cpu.clock();
                if self.bus().poll_input {
                    let pressed_keys: HashSet<Scancode> = event_pump.keyboard_state().pressed_scancodes().collect();
                    self.bus().controllers[0].buttons[0] = pressed_keys.contains(&Scancode::Right);
                    self.bus().controllers[0].buttons[1] = pressed_keys.contains(&Scancode::Left);
                    self.bus().controllers[0].buttons[2] = pressed_keys.contains(&Scancode::Down);
                    self.bus().controllers[0].buttons[3] = pressed_keys.contains(&Scancode::Up);
                    self.bus().controllers[0].buttons[4] = pressed_keys.contains(&Scancode::Return);
                    self.bus().controllers[0].buttons[5] = pressed_keys.contains(&Scancode::RShift);
                    self.bus().controllers[0].buttons[6] = pressed_keys.contains(&Scancode::Z);
                    self.bus().controllers[0].buttons[7] = pressed_keys.contains(&Scancode::X);
                }
            }
        }

        // The APU runs 6 times slower than the PPU
        if self.system_clock_counter % 6 == 0 {
            self.apu().clock();
        }

        if self.ppu().nmi {
            self.ppu().nmi = false;
            self.cpu.non_mask_interrupt_request();
        }

        if frame_complete {
            let pixels = self.ppu().frame.get_pixels();
            display::draw_frame(texture, canvas, &pixels);
            self.fps_limiter.limit(self.timer);
            self.timer = Instant::now();
            let mut lock = self.buffer.lock().expect("Error getting a lock for the buffer");
            lock.append(&mut self.cpu.bus.apu.buffer);
        }

        self.fps_limiter.calculate_fps();
        self.system_clock_counter += 1;
        
        frame_complete
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.system_clock_counter = 0;
    }

    pub fn bus(&mut self) -> &mut bus::Bus {
        &mut self.cpu.bus
    }

    pub fn ppu(&mut self) -> &mut ppu::ppu::Ppu2C02 {
        &mut self.cpu.bus.ppu
    }

    pub fn apu(&mut self) -> &mut audio::Apu2A03 {
        &mut self.cpu.bus.apu
    }

    fn dma_transfer(&mut self) {
        // The DMA is synchronized with every other clock cycle
        // Without loss of generality, we will do it every odd cycle
        if self.dma_dummy {
            if self.system_clock_counter % 2 == 1 {
                self.dma_dummy = false;
            }
        } else {
            if self.system_clock_counter % 2 == 0 {
                // Read data from cpu space
                let dma = self.bus().dma;
                let address = ((dma.page as u16) << 8) | (dma.address as u16);
                let data = self.cpu.bus.read(address);
                self.bus().dma.data = data;
            } else {
                // Write it to the ppu's OAM and increment DMA address
                let dma = self.cpu.bus.dma;
                self.ppu().oam.memory[dma.address as usize] = dma.data;
                self.bus().dma.address = dma.address.wrapping_add(1);

                // Since we're wrapping around, we know when it goes back to zero that it has written all 256 bytes
                if self.bus().dma.address == 0x00 {
                    self.bus().dma_transfer = false;
                    self.dma_dummy = true;
                }
            }
        }
    }
}