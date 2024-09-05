//! # Example: Hello world
//!
//! A simple hello world example displaying some primitive shapes and some text underneath.

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
use power_bank::screen::*;

use power_bank::{Data, PortData};

fn main() -> Result<(), std::convert::Infallible> {
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

    let output_settings = OutputSettingsBuilder::new()
    .theme(BinaryColorTheme::OledWhite)
    .build();
    Window::new("RUST POWER BANK", &output_settings).show_static(&screen.get_display());

    Ok(())
}
