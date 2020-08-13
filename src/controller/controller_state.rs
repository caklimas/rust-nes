use serde::{Serialize, Deserialize};

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct ControllerState(u8);
    impl Debug;

    pub a, set_a: 0;
    pub b, set_b: 1;
    pub select, set_select: 2;
    pub start, set_start: 3;
    pub up, set_up: 4;
    pub down, set_down: 5;
    pub left, set_left: 6;
    pub right, set_right: 7;

    pub get, _: 7, 0;
}