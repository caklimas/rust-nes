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
pub const CONTROLLER_TWO_INPUT: u16 = 0x4017;

// Direct Memory Access
pub const DMA_ADDRESS: u16 = 0x4014;

// Audio
pub const APU_PULSE_1_TIMER: u16 = 0x4000;
pub const APU_PULSE_1_LENGTH_COUNTER: u16 = 0x4001;
pub const APU_PULSE_1_ENVELOPE: u16 = 0x4002;
pub const APU_PULSE_1_SWEEP: u16 = 0x4003;
pub const APU_PULSE_2_TIMER: u16 = 0x4004;
pub const APU_PULSE_2_LENGTH_COUNTER: u16 = 0x4005;
pub const APU_PULSE_2_ENVELOPE: u16 = 0x4006;
pub const APU_PULSE_2_SWEEP: u16 = 0x4007;
pub const APU_NOISE_1: u16 = 0x400C;
pub const APU_NOISE_2: u16 = 0x400E;
pub const APU_DMC: u16 = 0x4013;
pub const APU_STATUS: u16 = 0x4015;
pub const APU_FRAME_COUNTER: u16 = 0x4017;