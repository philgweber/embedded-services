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
