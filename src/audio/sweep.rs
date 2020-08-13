use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Sweep {
    pub divider_counter: u16,
    pub enabled: bool,
    pub negate: bool,
    pub period: u16,
    pub reload: bool,
    pub shift_amount: u8
}