pub enum PulseTimer {
    VolumeEnvelope0 = (1 << 0),
    VolumeEnvelope1 = (1 << 1),
    VolumeEnvelope2 = (1 << 2),
    VolumeEnvelope3 = (1 << 3),
    ConstantVolume = (1 << 4),
    LengthCounterHalt = (1 << 5),
    Duty0 = (1 << 6), 
    Duty1 = (1 << 7)
}

pub enum PusleSweepUnit {
    Shift0 = (1 << 0),
    Shift1 = (1 << 1),
    Shift2 = (1 << 2),
    Negate = (1 << 3),
    Period0 = (1 << 4),
    Period1 = (1 << 5),
    Period2 = (1 << 6),
    Enabled = (1 << 7),
}

pub enum PulseLengthCounterLoad {
    TimerHigh0 = (1 << 0),
    TimerHigh1 = (1 << 1),
    TimerHigh2 = (1 << 2),
    LengthCounterLoad0 = (1 << 3),
    LengthCounterLoad1 = (1 << 4),
    LengthCounterLoad2 = (1 << 5),
    LengthCounterLoad3 = (1 << 6),
    LengthCounterLoad4 = (1 << 7)
}