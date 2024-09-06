use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};

use power_bank::screen::*;

use power_bank::{Data, PortData};


// Main is itself an async task as well.
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

    let mut screen = StandbyScreen::switch_into(screen);
    screen.draw_base_widget();
    screen.update(&data);

    Timer::after(Duration::from_millis(5000)).await;
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