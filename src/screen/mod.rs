use embedded_graphics::{
    mono_font::{ascii::*, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    image::Image,
    text::{Alignment, Text},
};
use profont::*;
use tinybmp::Bmp;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use crate::*;

pub mod icons;
pub use icons::*;

pub mod standby_screen;
pub use standby_screen::*;

pub mod working_screen;
pub use working_screen::*;

pub mod light_adjust_screen;
pub use light_adjust_screen::*;

pub mod about_screen;
pub use about_screen::*;

// 1 pixel = 0.75 point
// 16x29 pixels (19)
static PROFONT_24POINT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&PROFONT_24_POINT, BinaryColor::On);

// 12x22 pixels
static PROFONT_18POINT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&PROFONT_18_POINT, BinaryColor::On);

static PROFONT_14POINT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&PROFONT_14_POINT, BinaryColor::On);

// (9)
static PROFONT_12POINT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&PROFONT_12_POINT, BinaryColor::On);

static PROFONT_10POINT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&PROFONT_10_POINT, BinaryColor::On);

static PROFONT_9POINT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&PROFONT_9_POINT, BinaryColor::On);

static PROFONT_7POINT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&PROFONT_7_POINT, BinaryColor::On);

static MONOFONT_5X8_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&FONT_5X8, BinaryColor::On);

static MONOFONT_4X6_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&FONT_4X6, BinaryColor::On);

pub enum ScreenType {
    StandBy,
    Working,
    LightAdjust,
    None,
}

pub struct SimulatorInitScreen {
    pub display: SimulatorDisplay<BinaryColor>,
}

pub trait Screen {
    fn switch_into(old_screen: impl Screen) -> Self;

    fn draw_base_widget(&mut self);

    fn update(&mut self, data: &Data);

    fn get_display(self) -> SimulatorDisplay<BinaryColor>;
}

impl Screen for SimulatorInitScreen {

    fn switch_into(old_screen: impl Screen) -> Self {
        let display = old_screen.get_display();
        Self { display }
    }

    fn draw_base_widget(&mut self) {}

    fn update(&mut self, data: &Data) {
    }

    fn get_display(self) -> SimulatorDisplay<BinaryColor> {
        self.display
    }
}


impl SimulatorInitScreen {
    pub fn new() -> Self {
        Self{
            display: SimulatorDisplay::new(Size::new(128, 32)),
        }

    }

    pub fn show_static(&self) {
        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledWhite)
            .build();
        Window::new("RUST POWER BANK", &output_settings).show_static(&self.display);
    }
}
