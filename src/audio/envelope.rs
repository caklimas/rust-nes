#[derive(Debug, Default)]
pub struct Envelope {
    pub start: bool
}

impl Envelope {
    pub fn new() -> Self {
        Envelope {
            start: false
        }
    }

    pub fn clock(&mut self) {

    }
}