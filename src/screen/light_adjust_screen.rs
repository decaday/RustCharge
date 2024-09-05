use crate::screen::*;

pub struct LightAdjustScreen {
    display: SimulatorDisplay<BinaryColor>,
}

impl Screen for LightAdjustScreen {

    fn switch_into(old_screen: impl Screen) -> Self {
        let display = old_screen.get_display();
        Self { display }
    }

    fn draw_base_widget(&mut self) {
        let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();

        let _ = Rectangle::with_corners(Point::new(9, 15), Point::new(117, 31))
            .into_styled(border_stroke)
            .draw(&mut self.display);

        let _ = Text::new(
                "BRIGHTNESS",
                Point::new(10, 12),
                PROFONT_9POINT_STYLE,
            )
            .draw(&mut self.display);
    }

    fn update(&mut self, data: &Data) {
        let clean_area = Rectangle::with_corners(Point::new(13, 17), Point::new(113, 29));
        let _ = self.display.fill_solid(&clean_area, BinaryColor::Off);


        let _ = Text::with_alignment(
            &data.get_brightness_percentage_string() as _,
            Point::new(117, 12),
            PROFONT_14POINT_STYLE,
            Alignment::Right,
        )
        .draw(&mut self.display);

        let brightness = data.brightness_percentage;
        let fill_area = Rectangle::with_corners(
            Point::new(13, 17), 
            Point::new((13 + brightness * 1) as i32, 29));
        let _ = self.display.fill_solid(&fill_area, BinaryColor::On);
    }

    fn get_display(self) -> SimulatorDisplay<BinaryColor> {
        self.display
    }
}