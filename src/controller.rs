/// Provides information for the controller input
/// Keys are mapped as follows:
/// 0 - Right
/// 1 - Left
/// 2 - Down
/// 3 - Up
/// 4 - Start
/// 5 - Select
/// 6 - B
/// 7 - A
#[derive(Debug, Default)]
pub struct Controller {
    pub buttons: [bool; 8],
    pub state: u8
}

impl Controller {
    pub fn set_state(&mut self) {
        for i in 0..self.buttons.len() {
            let button = self.buttons[i];
            if button {
                self.state |= 1 << i;
            } else {
                self.state &= !(0 << i);
            }
        }
    }

    pub fn get_msb(&mut self) -> u8 {
        let msb = if self.state & 0x80 > 0 { 1 } else { 0 };
        self.state <<= 1;

        msb
    }
}