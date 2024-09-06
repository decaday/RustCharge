use crate::screen::*;

pub struct StandbyScreen {
    display: SimulatorDisplay<BinaryColor>,
}

impl Screen for StandbyScreen {

    fn switch_into(old_screen: impl Screen) -> Self {
        let display = old_screen.get_display();
        let mut screen = Self { display };
        screen.draw_base_widget();
        screen
    }

    fn draw_base_widget(&mut self) {
        let _ = self.display.clear(BinaryColor::Off);
    }

    fn update(&mut self, data: &Data) {
        let clean_area = Rectangle::with_corners(Point::new(48, 0), Point::new(127, 31));
        let _ = self.display.fill_solid(&clean_area, BinaryColor::Off);

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

    fn get_display(self) -> SimulatorDisplay<BinaryColor> {
        self.display
    }
}
