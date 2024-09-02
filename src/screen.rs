use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
};
use profont::PROFONT_24_POINT;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use std::{path::Display, sync::Arc};

use crate::Data;

pub enum ScreenType {
    StandBy,
    None,
}

pub struct Screen<T> {
    pub current_type: ScreenType,
    pub display: T,
}

impl<T: DrawTarget<Color=BinaryColor>> Screen<T> {
    pub fn draw_screen(&mut self, screen_type: ScreenType) {
        match screen_type {
            ScreenType::StandBy => {
                self.draw_stand_by_screen();
            }
            ScreenType::None => todo!(),
        }
    }

    pub fn update_screen(&mut self, screen_type: ScreenType, data: Data){
        match screen_type {
            ScreenType::StandBy => {
                self.update_stand_by_screen(data);
            }
            ScreenType::None => todo!(),
        }
    }

    fn draw_stand_by_screen(&mut self) {
        let character_style = MonoTextStyle::new(&PROFONT_24_POINT, BinaryColor::On);
        Text::new(
            "POWER BANK",
            Point::new(0, 20),
            character_style,
        )
        .draw(&mut self.display);
    }

    fn update_stand_by_screen(&self, data: Data) {
    }

}


impl Screen<SimulatorDisplay<BinaryColor>> {
    pub fn new() -> Self {
        Self{
            display: SimulatorDisplay::new(Size::new(128, 32)),
            current_type: ScreenType::None,
        }

    }

    pub fn show_static(&self) {
        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledWhite)
            .build();
        Window::new("Hello World", &output_settings).show_static(&self.display);
    }
}
