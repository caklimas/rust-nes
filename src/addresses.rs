// CPU
pub const CPU_ADDRESS_UPPER: u16 = 0x1FFF;

// Pattern
pub const PATTERN_ADDRESS_UPPER: u16 = 0x1FFF;

// Name table
pub const NAME_TABLE_ADDRESS_LOWER: u16 = 0x2000;
pub const NAME_TABLE_ADDRESS_UPPER: u16 = 0x3EFF;


// Palette
pub const PALETTE_ADDRESS_LOWER: u16 = 0x3F00;
pub const PALETTE_ADDRESS_UPPER: u16 = 0x3FFF;

// Attribute table
pub const ATTRIBUTE_TABLE_ADDRESS_LOWER: u16 = 0x23C0;

// PPU
pub const PPU_ADDRESS_START: u16 = 0x2000;
pub const PPU_ADDRESS_END: u16 = 0x3FFF;
pub const PPU_ADDRESS_RANGE: u16 = 0x0007;

// Controls
pub const CONTROLLER_ONE_INPUT: u16 = 0x4016;

// Direct Memory Access
pub const DMA_ADDRESS: u16 = 0x4014;

// Audio
pub const APU_PULSE_1_DUTY: u16 = 0x4000;
pub const APU_PULSE_1_SWEEP: u16 = 0x4001;
pub const APU_PULSE_1_TIMER_LOW: u16 = 0x4002;
pub const APU_PULSE_1_TIMER_HIGH: u16 = 0x4003;
pub const APU_PULSE_2_DUTY: u16 = 0x4004;
pub const APU_PULSE_2_SWEEP: u16 = 0x4005;
pub const APU_PULSE_2_TIMER_LOW: u16 = 0x4006;
pub const APU_PULSE_2_TIMER_HIGH: u16 = 0x4007;
pub const APU_TRIANGLE_COUNTER_RELOAD: u16 = 0x4008;
pub const APU_TRIANGLE_TIMER_LOW: u16 = 0x400A;
pub const APU_TRIANGLE_TIMER_HIGH: u16 = 0x400B;
pub const APU_NOISE_VOLUME: u16 = 0x400C;
pub const APU_NOISE_PERIOD: u16 = 0x400E;
pub const APU_NOISE_COUNTER_LOAD: u16 = 0x400F;
pub const APU_DMC: u16 = 0x4013;
pub const APU_STATUS: u16 = 0x4015;
pub const APU_FRAME_COUNTER: u16 = 0x4017;

pub fn get_address_range(address: u16) -> AddressRange {
    match address {
        0..=CPU_ADDRESS_UPPER => AddressRange::Cpu,
        PPU_ADDRESS_START..=PPU_ADDRESS_END => AddressRange::Ppu,
        DMA_ADDRESS => AddressRange::Dma,
        APU_PULSE_1_DUTY..=APU_DMC | APU_STATUS | APU_FRAME_COUNTER => AddressRange::Apu,
        CONTROLLER_ONE_INPUT => AddressRange::Controller,
        _ => AddressRange::Unknown
    }
}

pub enum AddressRange {
    Cpu,
    Ppu,
    Dma,
    Apu,
    Controller,
    Unknown
}