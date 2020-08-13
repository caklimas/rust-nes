use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Mirror {
    Hardware,
    Horizontal,
    Vertical,
    OneScreenLow,
    OneScreenHigh
}