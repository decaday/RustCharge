#![no_std]

pub mod screen;
pub mod display;
pub mod monitor;

#[cfg(feature = "device-arm")]
type String = heapless::String<40>;
