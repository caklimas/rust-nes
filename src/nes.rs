use crate::cpu;
use crate::bus;
use crate::ppu;
use crate::audio;

pub struct Nes {
    pub cpu: cpu::cpu::Cpu6502,
    system_clock_counter: u32,
    dma_dummy: bool,
    pub can_draw: bool
}

impl Nes {
    pub fn new() -> Self {
        Nes {
            cpu: cpu::cpu::Cpu6502::new(),
            system_clock_counter: 0,
            dma_dummy: false,
            can_draw: false
        }
    }

    pub fn clock(&mut self) {
        self.ppu().clock();
        self.apu().clock();

        // The CPU runs 3 times slower than the PPU
        if self.system_clock_counter % 3 == 0 {
            // If DMA transer is happening, then the cpu is suspended
            if self.bus().dma_transfer {
                self.dma_transfer();
            } else {
                self.cpu.clock();
            }
        }

        if self.ppu().nmi {
            self.ppu().nmi = false;
            self.cpu.non_mask_interrupt_request();
        }

        self.system_clock_counter += 1;
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

    pub fn apu(&mut self) -> &mut audio::apu::Apu {
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
                self.ppu().oam[dma.address as usize] = dma.data;
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