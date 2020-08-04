pub mod controller_state;

use std::collections::HashSet;
use sdl2::keyboard::{Scancode};

const CONTROLLER_OPEN_BUS: u8 = 0x40;

/// Provides information for the controller input
/// Keys are mapped as follows:
/// 0 - A
/// 1 - B
/// 2 - Select
/// 3 - Start
/// 4 - Up
/// 5 - Down
/// 6 - Left
/// 7 - Right
#[derive(Debug)]
pub struct Controller {
    pub button_states: u8,
    pub button_shift: u8,
    pub controller_state: controller_state::ControllerState
}

impl Controller {
    pub fn read(&mut self, poll_input: &u8) -> u8 {
        let bit = if self.button_shift < 8 {
            (self.controller_state.get() & (1 << self.button_shift)) >> self.button_shift
        } else {
            1
        };

        if poll_input & 1 != 0 {
            self.button_shift = 0;
        } else {
            self.button_shift += 1;
        }

        bit | CONTROLLER_OPEN_BUS
    }

    pub fn set_controller_state(&mut self, pressed_scancodes: HashSet<Scancode>) {
        self.controller_state.set_a(pressed_scancodes.contains(&Scancode::Z));
        self.controller_state.set_b(pressed_scancodes.contains(&Scancode::X));
        self.controller_state.set_start(pressed_scancodes.contains(&Scancode::Return));
        self.controller_state.set_up(pressed_scancodes.contains(&Scancode::Up));
        self.controller_state.set_down(pressed_scancodes.contains(&Scancode::Down));
        self.controller_state.set_left(pressed_scancodes.contains(&Scancode::Left));
        self.controller_state.set_right(pressed_scancodes.contains(&Scancode::Right));
    }

    pub fn write(&mut self, poll_input: &u8) {
        if poll_input & 1 == 0 {
            return;
        }

        self.button_shift = 0;
    }
}

impl Default for Controller {
    fn default() -> Self { 
        Controller {
            button_states: 0,
            button_shift: 0,
            controller_state: controller_state::ControllerState(0)
        }
    }
}