use crate::screen::*;

pub struct AboutScreen {
    display: DisplayType,
}

impl Screen for AboutScreen {

    fn switch_into(old_screen: impl Screen) -> Self {
        let display = old_screen.get_display();
        let mut screen = Self { display };
        screen.draw_base_widget();
        screen
    }

    fn draw_base_widget(&mut self) {
        let _ = self.display.clear(BinaryColor::Off);

        let _ = Text::with_alignment(
            "RustChargeV0.1",
            Point::new(63, 10),
            PROFONT_12POINT_STYLE,
            Alignment::Center,
        )
        .draw(&mut self.display);

        let _ = Text::new(
            "DesignedBy:Github@decaday",
            Point::new(0, 20),
            PROFONT_7POINT_STYLE,
        )
        .draw(&mut self.display);

        let _ = Text::new(
            "PoweredBy:RustLang",
            Point::new(0, 28),
            PROFONT_7POINT_STYLE,
        )
        .draw(&mut self.display);

        let bmp: Bmp<BinaryColor> = Bmp::from_slice(include_bytes!("../././assets/small-crab.bmp")).unwrap();
        let img = Image::new(&bmp, Point::new(96, 22));
        let _ = img.draw(&mut self.display);
    }

    fn update(&mut self, _data: &Data) {
        
    }

    fn get_display(self) -> DisplayType {
        self.display
    }
}
