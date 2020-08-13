use std::time::Instant;

pub struct InstantWrapper {
    pub instant: Instant
}

impl Default for InstantWrapper {
    fn default() -> Self {
        InstantWrapper {
            instant: Instant::now()
        }
    }
}