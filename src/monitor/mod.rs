use embassy_stm32::adc::{Adc, AdcChannel, Instance, Resolution, SampleTime};

use crate::*;

pub mod data;
pub use data::*;

pub fn init<'a, T: Instance>(adc: Adc<'a, T>,
    light_current: impl AdcChannel<T>, 
    powerbank_current: impl AdcChannel<T>,
    battery_voltage: impl AdcChannel<T>) {
}