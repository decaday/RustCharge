#![no_std]
#![no_main]
use {defmt_rtt as _, panic_probe as _};
use defmt::*;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
};

#[cfg(feature = "device-simulator")]
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};

use power_bank::screen::*;
use power_bank::display;
use power_bank::monitor;
use power_bank::{Data, PortData};


// bind_interrupts!(struct Irqs {
//     ADC1_2 => adc::InterruptHandler<ADC1>;
// });

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    use embassy_stm32::i2c::{Error, I2c};
    use embassy_stm32::adc::{Adc, Config, SampleTime};
    use embassy_stm32::time::Hertz;
    info!("Hello world!");
    let p = embassy_stm32::init(Default::default());
    let pin = p.PC0;

    let i2c = I2c::new_blocking(p.I2C1, p.PA12, p.PB8, Hertz(400_000), Default::default());
    let mut display = display::init(i2c).unwrap();

    let mut adc = Adc::new(p.ADC1);

    let mut vrefint = adc.enable_vref();
    
    monitor::init(adc, p.PA0, p.PA1, p.PA2, vrefint);
}

// Main is itself an async task as well.
#[cfg(feature = "device-simulator")]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Create a new simulator display with 128x64 pixels.
    let mut screen = SimulatorInitScreen::new();
    
    let port_data = PortData::Output(2455);
    let data = Data {
        brightness_percentage: 30,
        battery_percentage: 52,
        battery_voltage_mv: 3512,
        powerbank_current_ma: port_data.clone(),
        light_current_ma: port_data.clone(),
        // output1_voltage_mills_data: port_data.clone(),
        // output2_voltage_mills_data: port_data.clone(),
    };

    // let mut screen = StandbyScreen::switch_into(screen);
    let mut screen = AboutScreen::switch_into(screen); //StandbyScreen::switch_into(screen);

    screen.draw_base_widget();
    screen.update(&data);

    Timer::after(Duration::from_millis(5)).await;
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();
    Window::new("RUST POWER BANK", &output_settings).show_static(&screen.get_display());

    // spawner.spawn(blink(p.P0_13.degrade())).unwrap();

    loop {
        // // Asynchronously wait for GPIO events, allowing other tasks
        // // to run, or the core to sleep.
        // button.wait_for_low().await;
        // info!("Button pressed!");
        // button.wait_for_high().await;
        // info!("Button released!");
    }
}



// Declare async tasks
#[embassy_executor::task]
async fn blink() {
    // let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

    // loop {
    //     // Timekeeping is globally available, no need to mess with hardware timers.
    //     led.set_high();
    //     Timer::after_millis(150).await;
    //     led.set_low();
    //     Timer::after_millis(150).await;
    // }
}