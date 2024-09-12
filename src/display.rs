use embassy_stm32::i2c::I2c;
use embassy_stm32::mode::Blocking;
use embassy_stm32::Peripherals;
use embassy_stm32::time::Hertz;

use ssd1306::prelude::*;
use ssd1306::{mode::BufferedGraphicsMode, I2CDisplayInterface, Ssd1306};

#[cfg(feature = "device-simulator")]
pub type DisplayType = SimulatorDisplay<BinaryColor>;
#[cfg(feature = "device-arm")]
pub type DisplayType<'a> = Ssd1306<I2CInterface<I2c<'a, Blocking>>, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>;


pub fn init(i2c: I2c<Blocking>) -> Result<DisplayType, ()> {
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();
    display.init().unwrap();
    Ok(display)
}