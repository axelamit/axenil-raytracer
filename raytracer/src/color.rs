use crate::math::*;

pub enum Colors {
    Black,
    White,
    Red,
    Green,
    Blue,
}

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn new_from_color(color: Colors) -> Color {
        match color {
            Colors::Black => Color::new_from_rgba(0., 0., 0., 0.),
            Colors::White => Color::new_from_rgba(255., 255., 255., 0.),
            Colors::Red => Color::new_from_rgba(255., 0., 0., 0.),
            Colors::Green => Color::new_from_rgba(0., 255., 0., 0.),
            Colors::Blue => Color::new_from_rgba(0., 0., 255., 0.),
        }
    }

    pub fn new_from_rgba(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn add_color(&self, color: &Color) -> Color {
        Color {
            r: self.r + color.r,
            g: self.g + color.g,
            b: self.b + color.b,
            a: self.a + color.a,
        }
    }

    pub fn multiply_color(&self, color: &Color) -> Color {
        Color {
            r: self.r * color.r,
            g: self.g * color.g,
            b: self.b * color.b,
            a: self.a * color.a,
        }
    }

    pub fn multiply_scalar(&self, scalar: f64) -> Color {
        Color {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
            a: self.a * scalar,
        }
    }

    pub fn min_max(&self) -> Color {
        Color {
            r: min(255., max(0., self.r)),
            g: min(255., max(0., self.g)),
            b: min(255., max(0., self.b)),
            a: min(255., max(0., self.a)),
        }
    }

    pub fn normalize(&self) -> Color {
        self.multiply_scalar(1. / 255.)
    }

    pub fn to_normal_range(&self) -> Color {
        self.multiply_scalar(255.)
    }
}
