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

use power_bank::screen::{ScreenType, Screen};

use power_bank::{Data, PortData, PortState};

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new simulator display with 128x64 pixels.
    let mut screen = Screen::new();
    
    let port_data = PortData {
        state: PortState::Unuse,
        data: 0,
    };
    let data = Data {
        battery_percentage: 52,
        battery_voltage_mills_data: 3512,
        powerbank_current_ma: port_data.clone(),
        led_current_ma: port_data.clone(),
        output1_voltage_mills_data: port_data.clone(),
        output2_voltage_mills_data: port_data.clone(),
    };

    screen.draw_screen(ScreenType::StandBy);
    screen.update_screen(data);
    screen.show_static();

    Ok(())
}
