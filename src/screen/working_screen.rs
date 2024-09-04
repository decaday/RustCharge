use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
};
use profont::{PROFONT_24_POINT, PROFONT_18_POINT};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use super::*;

impl<T: DrawTarget<Color=BinaryColor>> Screen<T> {
    pub(super) fn draw_working_screen(&mut self) {

    }

    pub(super) fn update_working_screen(&mut self, data: &Data) {
        let clean_area = self.display.bounding_box();
        let _ = self.display.fill_solid(&clean_area, BinaryColor::Off);

        let _ = Text::new(
            &data.get_battery_percentage_string() as _,
            Point::new(0, 25),
            PROFONT_24POINT_STYLE,
        )
        .draw(&mut self.display);

        if let Some(powerbank_power) = data.get_powerbank_power_string(Some("I:"), Some("O:")) {
            let _ = Text::new(
                &powerbank_power as _,
                Point::new(52, 8),
                PROFONT_12POINT_STYLE,
            )
            .draw(&mut self.display);
        }

        if let Some(light_power) = data.get_light_power_string(None, Some("L:")) {
            let _ = Text::new(
                &light_power as _,
                Point::new(52, 19),
                PROFONT_12POINT_STYLE,
            )
            .draw(&mut self.display);
        }

        let _ = Text::new(
            &data.get_battery_voltage_string() as _,
            Point::new(52, 30),
            PROFONT_12POINT_STYLE,
        )
        .draw(&mut self.display);
    }

}