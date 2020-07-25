pub mod apu;
pub mod device;
pub mod envelope;
pub mod filter;
pub mod noise;
pub mod pulse;
pub mod sweep;
pub mod triangle;

const LENGTH_COUNTER_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12,
    16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30
];

const SAMPLE_RATE: i32 = 44_100;