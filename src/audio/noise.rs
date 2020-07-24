use super::envelope;

#[derive(Debug, Default)]
pub struct Noise {
    pub envelope: envelope::Envelope,
    pub length_counter_halt: bool
}

impl Noise {
    pub fn clock(&mut self) -> u16 {
        0
    }
}