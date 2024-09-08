use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Icon {
    Charging,
    DoubleThunder,
    Thunder,
    Unuse,
}

pub fn next(position: Point) -> Point {
    let mut new_position = position;
    if position.y == 16 {
        new_position.y = 0;
        new_position.x += 16;
    } else {
        new_position.y += 16;
    }
    new_position
}

pub fn draw_icons_with_position(display: &mut impl DrawTarget<Color = BinaryColor>, top_left: Point, icons: [Icon; 3]) {
    let clean_area = Rectangle::new(top_left, Size::new(32, 32));
    let _ = display.fill_solid(&clean_area, BinaryColor::Off);

    let mut position = top_left;
    for icon in icons {
        match icon {
            Icon::Charging => {
                let bmp: Bmp<BinaryColor> = Bmp::from_slice(include_bytes!("../././assets/charging.bmp")).unwrap();
                let img = Image::new(&bmp, position);
                position = next(position);
                let _ = img.draw(display);
            },
            Icon::DoubleThunder => {
                let bmp: Bmp<BinaryColor> = Bmp::from_slice(include_bytes!("../././assets/double-thunder.bmp")).unwrap();
                let img = Image::new(&bmp, position);
                position = next(position);
                let _ = img.draw(display);
            },
            Icon::Thunder => {
                let bmp: Bmp<BinaryColor> = Bmp::from_slice(include_bytes!("../././assets/thunder.bmp")).unwrap();
                let img = Image::new(&bmp, position);
                position = next(position);
                let _ = img.draw(display);
            },
            Icon::Unuse => {},
        }
    }
}

pub fn draw_icons(display: &mut impl DrawTarget<Color = BinaryColor>, icons: [Icon; 3]) {
    let top_left = Point::new(95, 0);
    draw_icons_with_position(display, top_left, icons);
}