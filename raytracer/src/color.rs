pub enum Colors{
    Black,
    White,
    Red,
    Green,
    Blue,
}

pub struct Color{
    r: u32,
    g: u32,
    b: u32,
    a: u32,
}

impl Color{
    pub fn new_from_color(color: Colors) -> Color{
        match color{
            Colors::Black => Color::new_from_rgba(0, 0, 0, 0),
            Colors::White => Color::new_from_rgba(255, 255, 255, 0),
            Colors::Red => Color::new_from_rgba(255, 0, 0, 0),
            Colors::Green => Color::new_from_rgba(0, 255, 0, 0),
            Colors::Blue => Color::new_from_rgba(0, 0, 255, 0),
        }
    }

    pub fn new_from_rgba(r: u32, g: u32, b: u32, a: u32) -> Color{
        Color{
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}
