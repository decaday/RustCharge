use crate::screen::*;

pub struct WorkingScreen<'a> {
    display: DisplayType<'a>,
}

impl<'a> Screen for WorkingScreen<'a> {

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
        let clean_area = self.display.bounding_box();
        let _ = self.display.fill_solid(&clean_area, BinaryColor::Off);

        let _ = Text::new(
            &data.get_battery_percentage_string() as _,
            Point::new(0, 25),
            PROFONT_24POINT_STYLE,
        )
        .draw(&mut self.display);

        if let Some(powerbank_power) = data.get_powerbank_power_string(Some("I"), Some("o")) {
            let _ = Text::new(
                &powerbank_power as _,
                Point::new(52, 8),
                PROFONT_10POINT_STYLE,
            )
            .draw(&mut self.display);
        }

        if let Some(light_power) = data.get_light_power_string(None, Some("L")) {
            let _ = Text::new(
                &light_power as _,
                Point::new(52, 19),
                PROFONT_10POINT_STYLE,
            )
            .draw(&mut self.display);
        }

        let _ = Text::new(
            &data.get_battery_voltage_string() as _,
            Point::new(52, 30),
            PROFONT_10POINT_STYLE,
        )
        .draw(&mut self.display);

        icons::draw_icons(&mut self.display, data.get_icons_list());
    }

    fn get_display(self) -> DisplayType<'a> {
        self.display
    }
}