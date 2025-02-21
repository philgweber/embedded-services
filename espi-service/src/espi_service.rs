use embassy_sync::{blocking_mutex::Mutex, once_lock::OnceLock};

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embedded_services::{
    comms::{self, EndpointID, External},
    info,
};

use core::cell::RefCell;

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Capabilities {
    pub events: u16,
    pub fw_version: super::Version,
    pub secure_state: u8,
    pub boot_status: u8,
    pub fan_mask: u8,
    pub battery_mask: u8,
    pub temp_mask: u16,
    pub key_mask: u16,
    pub debug_mask: u16,
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Battery {
    pub events: u16,
    pub last_full_charge: u32,
    pub cycle_count: u32,
    pub state: u32,
    pub present_rate: u32,
    pub remain_cap: u32,
    pub present_volt: u32,
    pub psr_state: u32,
    pub psr_max_out: u32,
    pub psr_max_in: u32,
    pub peak_level: u32,
    pub peak_power: u32,
    pub sus_level: u32,
    pub sus_power: u32,
    pub peak_thres: u32,
    pub sus_thres: u32,
    pub trip_thres: u32,
    pub bmc_data: u32,
    pub bmd_data: u32,
    pub bmd_flags: u32,
    pub bmd_count: u32,
    pub charge_time: u32,
    pub run_time: u32,
    pub sample_time: u32,
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Thermal {
    pub events: u16,
    pub cool_mode: u32,
    pub dba_limit: u32,
    pub sonne_limit: u32,
    pub ma_limit: u32,
    pub fan1_on_temp: u32,
    pub fan1_ramp_temp: u32,
    pub fan1_max_temp: u32,
    pub fan1_crt_temp: u32,
    pub fan1_hot_temp: u32,
    pub fan1_max_rpm: u32,
    pub fan1_cur_rpm: u32,
    pub tmp1_val: u32,
    pub tmp1_timeout: u32,
    pub tmp1_low: u32,
    pub tmp1_high: u32,
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct TimeAlarm {
    pub events: u16,
    pub capability: u32,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub valid: u8,
    pub daylight: u8,
    pub res1: u8,
    pub milli: u16,
    pub time_zone: u16,
    pub res2: u16,
    pub alarm_status: u32,
    pub ac_time_val: u32,
    pub dc_time_val: u32,
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct MemoryMap {
    pub ver: super::Version,
    pub caps: Capabilities,
    pub batt: Battery,
    pub therm: Thermal,
    pub tas: TimeAlarm,
}

pub struct Service {
    pub endpoint: comms::Endpoint,
}

impl Service {
    pub fn new() -> Self {
        Service {
            endpoint: comms::Endpoint::uninit(EndpointID::External(External::Host)),
        }
    }
}

impl comms::MailboxDelegate for Service {
    fn receive(&self, message: &comms::Message) {
        if let Some(msg) = message.data.get::<super::BatteryMessage>() {
            update_battery_section(msg);
        }
        else if let Some(msg) = message.data.get::<super::ThermalMessage>() {
            update_thermal_section(msg);
        }
    }
}

static ESPI_SERVICE: OnceLock<Service> = OnceLock::new();
static MEMORY_MAP: OnceLock<Mutex<ThreadModeRawMutex, RefCell<&mut MemoryMap>>> = OnceLock::new();

// Initialize eSPI service and register it with the transport service
pub async fn init() {
    MEMORY_MAP.try_get().unwrap().lock(|memory_map| {
        let mut memory_map = memory_map.borrow_mut();
        memory_map.ver.major = 0;
        memory_map.ver.minor = 1;
        memory_map.ver.spin = 0;
        memory_map.ver.res0 = 0;
    });

    let espi_service = ESPI_SERVICE.get_or_init(|| Service::new());

    comms::register_endpoint(espi_service, &espi_service.endpoint)
        .await
        .unwrap();
}

fn update_battery_section(msg: &super::BatteryMessage) {
    MEMORY_MAP.try_get().unwrap().lock(|memory_map| {
        let mut memory_map = memory_map.borrow_mut();
        match msg {
            super::BatteryMessage::Events(events) => memory_map.batt.events = *events,
            super::BatteryMessage::LastFullCharge(last_full_charge) => {
                memory_map.batt.last_full_charge = *last_full_charge
            }
            super::BatteryMessage::CycleCount(cycle_count) => memory_map.batt.cycle_count = *cycle_count,
            super::BatteryMessage::State(state) => memory_map.batt.state = *state,
            super::BatteryMessage::PresentRate(present_rate) => memory_map.batt.present_rate = *present_rate,
            super::BatteryMessage::RemainCap(remain_cap) => memory_map.batt.remain_cap = *remain_cap,
            super::BatteryMessage::PresentVolt(present_volt) => memory_map.batt.present_volt = *present_volt,
            super::BatteryMessage::PsrState(psr_state) => memory_map.batt.psr_state = *psr_state,
            super::BatteryMessage::PsrMaxOut(psr_max_out) => memory_map.batt.psr_max_out = *psr_max_out,
            super::BatteryMessage::PsrMaxIn(psr_max_in) => memory_map.batt.psr_max_in = *psr_max_in,
            super::BatteryMessage::PeakLevel(peek_level) => memory_map.batt.peak_level = *peek_level,
            super::BatteryMessage::PeakPower(peek_power) => memory_map.batt.peak_power = *peek_power,
            super::BatteryMessage::SusLevel(sus_level) => memory_map.batt.sus_level = *sus_level,
            super::BatteryMessage::SusPower(sus_power) => memory_map.batt.sus_power = *sus_power,
            super::BatteryMessage::PeakThres(peek_thres) => memory_map.batt.peak_thres = *peek_thres,
            super::BatteryMessage::SusThres(sus_thres) => memory_map.batt.sus_thres = *sus_thres,
            super::BatteryMessage::TripThres(trip_thres) => memory_map.batt.trip_thres = *trip_thres,
            super::BatteryMessage::BmcData(bmc_data) => memory_map.batt.bmc_data = *bmc_data,
            super::BatteryMessage::BmdData(bmd_data) => memory_map.batt.bmd_data = *bmd_data,
            super::BatteryMessage::BmdFlags(bmd_flags) => memory_map.batt.bmd_flags = *bmd_flags,
            super::BatteryMessage::BmdCount(bmd_count) => memory_map.batt.bmd_count = *bmd_count,
            super::BatteryMessage::ChargeTime(charge_time) => memory_map.batt.charge_time = *charge_time,
            super::BatteryMessage::RunTime(run_time) => memory_map.batt.run_time = *run_time,
            super::BatteryMessage::SampleTime(sample_time) => memory_map.batt.sample_time = *sample_time,
        }
    });
}

fn update_thermal_section(msg: &super::ThermalMessage) {
    MEMORY_MAP.try_get().unwrap().lock(|memory_map| {
        let mut memory_map = memory_map.borrow_mut();
        match msg {
            super::ThermalMessage::Events(events) => memory_map.therm.events = *events,
            super::ThermalMessage::CoolMode(cool_mode) => memory_map.therm.cool_mode = *cool_mode,
            super::ThermalMessage::DbaLimit(dba_limit) => memory_map.therm.dba_limit = *dba_limit,
            super::ThermalMessage::SonneLimit(sonne_limit) => memory_map.therm.sonne_limit = *sonne_limit,
            super::ThermalMessage::MaLimit(ma_limit) => memory_map.therm.ma_limit = *ma_limit,
            super::ThermalMessage::Fan1OnTemp(fan1_on_temp) => memory_map.therm.fan1_on_temp = *fan1_on_temp,
            super::ThermalMessage::Fan1RampTemp(fan1_ramp_temp) => memory_map.therm.fan1_ramp_temp = *fan1_ramp_temp,
            super::ThermalMessage::Fan1MaxTemp(fan1_max_temp) => memory_map.therm.fan1_max_temp = *fan1_max_temp,
            super::ThermalMessage::Fan1CrtTemp(fan1_crt_temp) => memory_map.therm.fan1_crt_temp = *fan1_crt_temp,
            super::ThermalMessage::Fan1HotTemp(fan1_hot_temp) => memory_map.therm.fan1_hot_temp = *fan1_hot_temp,
            super::ThermalMessage::Fan1MaxRpm(fan1_max_rpm) => memory_map.therm.fan1_max_rpm = *fan1_max_rpm,
            super::ThermalMessage::Fan1CurRpm(fan1_cur_rpm) => memory_map.therm.fan1_cur_rpm = *fan1_cur_rpm,
            super::ThermalMessage::Tmp1Val(tmp1_val) => memory_map.therm.tmp1_val = *tmp1_val,
            super::ThermalMessage::Tmp1Timeout(tmp1_timeout) => memory_map.therm.tmp1_timeout = *tmp1_timeout,
            super::ThermalMessage::Tmp1Low(tmp1_low) => memory_map.therm.tmp1_low = *tmp1_low,
            super::ThermalMessage::Tmp1High(tmp1_high) => memory_map.therm.tmp1_high = *tmp1_high,
        }
    });
}

use embassy_imxrt::espi;

#[embassy_executor::task]
pub async fn espi_service(mut espi: espi::Espi<'static>, memory_map_buffer: &'static mut [u8]) {
    info!("Reserved memory map buffer size: {}", memory_map_buffer.len());
    info!("MemoryMap size: {}", size_of::<MemoryMap>());

    if size_of::<MemoryMap>() > memory_map_buffer.len() {
        panic!("MemoryMap is too bug for reserved memory buffer ");
    }

    memory_map_buffer.fill(0);

    let memory_map: &mut MemoryMap = unsafe { core::mem::transmute(memory_map_buffer.as_mut_ptr()) };
    let res = MEMORY_MAP.init(Mutex::new(RefCell::new(memory_map)));

    if res.is_err() {
        panic!("Failed to initialize MemoryMap");
    }

    init().await;

    info!("Initializing eSPI service");

    loop {
        embassy_time::Timer::after_secs(10).await;

        defmt::info!("------------------------------------------------------------ waiting for event");
        let event = espi.wait_for_event().await;
        match event {
            Ok(espi::Event::Port0(_port_event)) => {
                defmt::info!("Port 0");
                espi.complete_port(0).await;
            }
            Ok(espi::Event::Port1(_)) => {
                defmt::info!("Port 1");
            }
            Ok(espi::Event::Port2(_port_event)) => {
                defmt::info!("Port 2");
            }
            Ok(espi::Event::Port3(_)) => {
                defmt::info!("Port 3");
            }
            Ok(espi::Event::Port4(_)) => {
                defmt::info!("Port 4");
            }
            Ok(espi::Event::Port80) => {
                defmt::info!("Port 80");
            }
            Ok(espi::Event::WireChange) => {
                defmt::info!("WireChange");
            }
            Err(_) => {
                defmt::error!("Failed");
            }
        }
    }
}
