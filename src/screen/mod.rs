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

use std::{path::Display, sync::Arc};
// 1 pixel = 0.75 point
// 16x29 pixels (19)
static PROFONT_24POINT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&PROFONT_24_POINT, BinaryColor::On);

// 12x22 pixels
static PROFONT_18POINT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&PROFONT_18_POINT, BinaryColor::On);


use crate::Data;

pub enum ScreenType {
    StandBy,
    None,
}

pub struct Screen<T> {
    pub current_type: ScreenType,
    pub display: T,
}

// pub trait Screen {
//     fn switch_into(old_screen: impl Screen) -> Self ;

//     fn draw_base_widget(&mut self);

//     fn update(&mut self, data: Data);

//     fn get_display(&mut self) -> xxx;
// }

impl<T: DrawTarget<Color=BinaryColor>> Screen<T> {
    pub fn draw_screen(&mut self, screen_type: ScreenType) {
        match screen_type {
            ScreenType::StandBy => {
                self.draw_stand_by_screen();
                self.current_type = ScreenType::StandBy;
            }
            ScreenType::None => todo!(),
        }
    }

    pub fn update_screen(&mut self, data: Data){
        match self.current_type {
            ScreenType::StandBy => {
                self.update_stand_by_screen(data);
            }
            ScreenType::None => todo!(),
        }
    }

    fn draw_stand_by_screen(&mut self) {

    }

    fn update_stand_by_screen(&mut self, data: Data) {
        let _ = Text::new(
            &data.get_battery_percentage_string() as _,
            Point::new(48, 19),
            PROFONT_24POINT_STYLE,
        )
        .draw(&mut self.display);

        let a = &data.get_battery_percentage_string() as &str;
        println!("{a}");

        let _ = Text::new(
            &data.get_battery_voltage_string() as _,
            Point::new(48, 30),
            PROFONT_18POINT_STYLE,
        )
        .draw(&mut self.display);
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
