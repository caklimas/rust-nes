pub mod apu;
pub mod device;
pub mod envelope;
pub mod pulse;
pub mod sweep;

const LENGTH_COUNTER_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12,
    16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30
];

pub fn get_angular_frequency(hertz: f32) -> f32 {
    hertz * 2.0 * std::f32::consts::PI
}