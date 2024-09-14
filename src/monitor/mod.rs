use core::sync::atomic::AtomicU32;

use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_stm32::adc::{Adc, AdcChannel, Instance};
use embassy_stm32::peripherals::*;
use embassy_time::Timer;
use crate::*;

pub mod data;
pub use data::*;

const VREF_VOL_MV: u16 = 1200;

const POWERBANK_CURRENT_CRITICAL_MA: u32 = 20;
const LIGHT_CURRENT_CRITICAL_MA: u32 = 20;

const POWERBANK_CURRENT_SCALE: u16 = 20;
const LIGHT_CURRENT_SCALE: u16 = 20;

const POWERBANK_CURRENT_SAMPLE_RES_MOHM: u16 = 5;
const LIGHT_CURRENT_SAMPLE_RES_MOHM: u16 = 10;

const COULOMB_METER_PERIOD_MS: u32 = 10;

static MONITOR_DATA: Mutex<ThreadModeRawMutex, Option<Data>> = Mutex::new(None);
type MonitorType = Mutex<ThreadModeRawMutex, Option<Monitor>>;
static MONITOR: MonitorType = Mutex::new(None);
static COULOMB: AtomicU32 = AtomicU32::new(0);

pub struct Monitor {
    pub adc: Adc<'static, ADC1>,
    pub vref_channel: PA3,
    // pub battery_voltage_channel: PA0,
    pub light_current_channel: PA1,
    pub powerbank_current_channel: PA2,
}

pub async fn init(adc: Adc<'static, ADC1>,
    vref_channel: PA3,
    // mut battery_voltage_channel: PA0,
    light_current_channel: PA1, 
    powerbank_current_channel: PA2,
) {
    let monitor = Monitor {
        adc,
        vref_channel,
        light_current_channel,
        powerbank_current_channel,
    };
    {
        (*MONITOR.lock().await) = Some(monitor);
    }
}

#[embassy_executor::task]
pub async fn monitor_task(monitor_mutex: &'static MonitorType) {
    let mut data = Data::default();

    loop {
        let mut monitor_unlocked = monitor_mutex.lock().await;
        let monitor = monitor_unlocked.as_mut().unwrap();
        let battery_voltage_mv = get_avcc_mv(monitor.adc.read(&mut monitor.vref_channel).await, VREF_VOL_MV);
        let powerbank_current_ma = get_ma(get_mv(monitor.adc.read(&mut monitor.light_current_channel).await, battery_voltage_mv),
                                        battery_voltage_mv/2,
                                        POWERBANK_CURRENT_SCALE,
                                        POWERBANK_CURRENT_SAMPLE_RES_MOHM);
        let light_current_ma = get_ma(get_mv(monitor.adc.read(&mut monitor.light_current_channel).await, battery_voltage_mv),
                                        0,
                                        LIGHT_CURRENT_SCALE,
                                        LIGHT_CURRENT_SAMPLE_RES_MOHM); 

        data.battery_voltage_mv = battery_voltage_mv as u32;
        data.powerbank_current_ma = PortData::new(powerbank_current_ma, POWERBANK_CURRENT_CRITICAL_MA);
        data.light_current_ma = PortData::new(light_current_ma, LIGHT_CURRENT_CRITICAL_MA);

        Timer::after_millis(100).await;
    }
}



#[embassy_executor::task]
pub async fn coulomb_meter_task(monitor_mutex: &'static MonitorType) {
    let mut data = Data::default();

    loop {
        let mut monitor_unlocked = monitor_mutex.lock().await;
        let monitor = monitor_unlocked.as_mut().unwrap();
        let battery_voltage_mv = get_avcc_mv(monitor.adc.read(&mut monitor.vref_channel).await, VREF_VOL_MV);
        let powerbank_current_ma = get_ma(get_mv(monitor.adc.read(&mut monitor.light_current_channel).await, battery_voltage_mv),
                                        battery_voltage_mv/2,
                                        POWERBANK_CURRENT_SCALE,
                                        POWERBANK_CURRENT_SAMPLE_RES_MOHM);
        let light_current_ma = get_ma(get_mv(monitor.adc.read(&mut monitor.light_current_channel).await, battery_voltage_mv),
                                        0,
                                        LIGHT_CURRENT_SCALE,
                                        LIGHT_CURRENT_SAMPLE_RES_MOHM); 

        data.battery_voltage_mv = battery_voltage_mv as u32;
        data.powerbank_current_ma = PortData::new(powerbank_current_ma, POWERBANK_CURRENT_CRITICAL_MA);
        data.light_current_ma = PortData::new(light_current_ma, LIGHT_CURRENT_CRITICAL_MA);

        Timer::after_millis(COULOMB_METER_PERIOD_MS.into()).await;
    }
}


// for 12bits ADC
pub fn get_avcc_mv(vref_result: u16, vref_mv: u16) -> u16 {
    (vref_mv * 4095 / vref_result) as u16
}


pub fn get_mv(adc_result: u16, avcc_mv: u16) -> u16 {
    (adc_result * 4095 / avcc_mv) as u16
}

pub fn get_ma(channel_mv: u16, baseline_mv: u16, amplifier_scale: u16, sample_resistor_mohm: u16) -> i32 {
    let delta_mv:i32 = channel_mv as i32 - baseline_mv as i32;
    let delta_ma = (delta_mv as i32 * sample_resistor_mohm as i32) / amplifier_scale as i32 / 1000;
    delta_ma as i32
}
