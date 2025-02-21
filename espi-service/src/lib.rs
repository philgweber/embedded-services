#![no_std]

pub mod espi_service;

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Version {
    major: u8,
    minor: u8,
    spin: u8,
    res0: u8,
}

#[derive(Copy, Clone, Debug)]
pub enum CapabilitiesMessage {
    Events(u16),
    FwVersion(Version),
    SecureState(u8),
    BootStatus(u8),
    FanMask(u8),
    BatteryMask(u8),
    TempMask(u16),
    KeyMask(u16),
    DebugMask(u16),
}

#[derive(Copy, Clone, Debug)]
pub enum BatteryMessage {
    Events(u16),
    LastFullCharge(u32),
    CycleCount(u32),
    State(u32),
    PresentRate(u32),
    RemainCap(u32),
    PresentVolt(u32),
    PsrState(u32),
    PsrMaxOut(u32),
    PsrMaxIn(u32),
    PeakLevel(u32),
    PeakPower(u32),
    SusLevel(u32),
    SusPower(u32),
    PeakThres(u32),
    SusThres(u32),
    TripThres(u32),
    BmcData(u32),
    BmdData(u32),
    BmdFlags(u32),
    BmdCount(u32),
    ChargeTime(u32),
    RunTime(u32),
    SampleTime(u32),
}

// #[repr(C, packed)]
// #[derive(Default, Copy, Clone, Debug)]
// pub struct Thermal {
//     pub events: u16,
//     pub cool_mode: u32,
//     pub dba_limit: u32,
//     pub sonne_limit: u32,
//     pub ma_limit: u32,
//     pub fan1_on_temp: u32,
//     pub fan1_ramp_temp: u32,
//     pub fan1_max_temp: u32,
//     pub fan1_crt_temp: u32,
//     pub fan1_hot_temp: u32,
//     pub fan1_max_rpm: u32,
//     pub fan1_cur_rpm: u32,
//     pub tmp1_val: u32,
//     pub tmp1_timeout: u32,
//     pub tmp1_low: u32,
//     pub tmp1_high: u32,
// }

pub enum ThermalMessage {
    Events(u16),
    CoolMode(u32),
    DbaLimit(u32),
    SonneLimit(u32),
    MaLimit(u32),
    Fan1OnTemp(u32),
    Fan1RampTemp(u32),
    Fan1MaxTemp(u32),
    Fan1CrtTemp(u32),
    Fan1HotTemp(u32),
    Fan1MaxRpm(u32),
    Fan1CurRpm(u32),
    Tmp1Val(u32),
    Tmp1Timeout(u32),
    Tmp1Low(u32),
    Tmp1High(u32),
}
