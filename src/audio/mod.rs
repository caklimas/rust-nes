pub mod apu;
pub mod device;
pub mod pulse;
pub mod sine;
pub mod square;

pub fn get_angular_frequency(hertz: f32) -> f32 {
    hertz * 2.0 * std::f32::consts::PI
}