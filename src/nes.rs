use sdl2::keyboard::{Scancode};
use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::{Instant};

use crate::audio;
use crate::bus;
use crate::cpu;
use crate::display;
use crate::instant::InstantWrapper;
use crate::ppu;

#[derive(Serialize, Deserialize)]
pub struct Nes {
    pub cpu: cpu::Cpu6502,
    buffer: Arc<Mutex<Vec<f32>>>,
    dma_dummy: bool,
    fps_limiter: ppu::fps_limiter::FpsLimiter,
    system_clock_counter: u32,
    #[serde(skip_serializing, skip_deserializing)]
    timer: InstantWrapper
}

impl Nes {
    pub fn new(buffer: Arc<Mutex<Vec<f32>>>) -> Self {
        Nes {
            cpu: cpu::Cpu6502::new(),
            buffer,
            dma_dummy: false,
            fps_limiter: ppu::fps_limiter::FpsLimiter::new(60),
            system_clock_counter: 0,
            timer: Default::default()
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

        if self.apu().trigger_interrupt && self.cpu.get_flag(cpu::Flags6502::DisableInterrupts) == 0 {
            self.cpu.interrupt_request();
            self.apu().trigger_interrupt = false;
        }

        self.check_mapper_irq();

        if frame_complete {
            let pixels = self.ppu().frame.get_pixels();
            display::draw_frame(texture, canvas, &pixels);
            self.fps_limiter.limit(&self.timer);
            self.timer.instant = Instant::now();
            let mut lock = self.buffer.lock().expect("Error getting a lock for the buffer");
            lock.append(&mut self.cpu.bus.apu.buffer);
        }

        if self.bus().strobe_pulse & 1 == 1 {
            let pressed_scancodes: HashSet<Scancode> = event_pump.keyboard_state().pressed_scancodes().collect();
            self.bus().controllers[0].set_controller_state(pressed_scancodes);
        }

        self.fps_limiter.calculate_fps();
        self.system_clock_counter += 1;
        
        frame_complete
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.system_clock_counter = 0;
    }

    pub fn load_buffer(&mut self, buffer: Arc<Mutex<Vec<f32>>>) {
        self.buffer = buffer;
    }

    pub fn bus(&mut self) -> &mut bus::Bus {
        &mut self.cpu.bus
    }

    pub fn ppu(&mut self) -> &mut ppu::Ppu2C02 {
        &mut self.cpu.bus.ppu
    }

    pub fn apu(&mut self) -> &mut audio::Apu2A03 {
        &mut self.cpu.bus.apu
    }

    fn check_mapper_irq(&mut self) {
        let mut trigger_interrupt = false;
        if let Some(ref mut c) = self.cpu.bus.cartridge {
            if let Some(ref mut m) = c.borrow_mut().mapper {
                if m.irq_active() {
                    trigger_interrupt = true;
                    m.irq_clear();
                }
            }
        }

        if trigger_interrupt && self.cpu.get_flag(cpu::Flags6502::DisableInterrupts) == 0 {
            self.cpu.interrupt_request();
        }
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