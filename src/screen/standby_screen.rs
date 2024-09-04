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

use crate::screen::*;

impl<T: DrawTarget<Color=BinaryColor>> Screen<T> {
    pub(super) fn draw_stand_by_screen(&mut self) {

    }

    pub(super) fn update_stand_by_screen(&mut self, data: &Data) {
        let clean_area = Rectangle::with_corners(Point::new(48, 0), Point::new(127, 31));
        self.display.fill_solid(&clean_area, BinaryColor::Off);

        let _ = Text::new(
            &data.get_battery_percentage_string() as _,
            Point::new(48, 19),
            PROFONT_24POINT_STYLE,
        )
        .draw(&mut self.display);

        let _ = Text::new(
            &data.get_battery_voltage_string() as _,
            Point::new(48, 30),
            PROFONT_18POINT_STYLE,
        )
        .draw(&mut self.display);
    }

}