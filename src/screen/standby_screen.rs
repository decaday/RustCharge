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
        
        let bmp: Bmp<BinaryColor> = Bmp::from_slice(include_bytes!("../././assets/rust-battery.bmp")).unwrap();
        let img = Image::new(&bmp, Point::new(0, 0));
        let _ = img.draw(&mut self.display);
    }

    fn update(&mut self, data: &Data) {
        let clean_area = Rectangle::with_corners(Point::new(48, 0), Point::new(127, 31));
        let _ = self.display.fill_solid(&clean_area, BinaryColor::Off);

        let _ = Text::new(
            &data.get_battery_percentage_string() as _,
            Point::new(40, 19),
            PROFONT_24POINT_STYLE,
        )
        .draw(&mut self.display);

        let _ = Text::new(
            &data.get_battery_voltage_string() as _,
            Point::new(40, 30),
            PROFONT_12POINT_STYLE,
        )
        .draw(&mut self.display);

        icons::draw_icons(&mut self.display, data.get_icons_list());
    }

    fn get_display(self) -> SimulatorDisplay<BinaryColor> {
        self.display
    }
}
