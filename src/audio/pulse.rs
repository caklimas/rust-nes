bitfield! {
    pub struct PulseDuty(u8);
    impl Debug;

    pub u8, envelope_period_volume, _: 3, 0;
    pub u8, constant_volume, _: 4;
    pub u8, loop_envelope, _: 5;
    pub u8, duty, _: 7, 6;
    pub u8, get, set: 7, 0;
}

bitfield! {
    pub struct PulseSweep(u8);
    impl Debug;

    pub u8, shift_count, _: 2, 0;
    pub u8, negative, _: 3;
    pub u8, period, _: 6, 4;
    pub u8, enabled, _: 7;
    pub u8, get, set: 7, 0;
}

bitfield! {
    pub struct PulseTimerLow(u8);
    impl Debug;

    pub u8, get, set: 7, 0;
}

bitfield! {
    pub struct PulseTimerHigh(u8);
    impl Debug;

    pub u8, timer_high, _: 2, 0;
    pub u8, length_counter_load, _: 7, 3;
    pub u8, get, set: 7, 0;
}