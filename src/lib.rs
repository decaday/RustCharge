#![no_std]

use embassy_time::Timer;

pub mod screen;
pub mod display;
pub mod monitor;

#[cfg(feature = "device-arm")]
type String = heapless::String<40>;


#[embassy_executor::task]
pub async fn powerbank_task() {
    loop {
        Timer::after_millis(10).await;
    }
}