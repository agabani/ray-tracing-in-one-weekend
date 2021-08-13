use std::fmt::{Display, Formatter};

pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let r = (255.99 * self.r) as u8;
        let g = (255.99 * self.g) as u8;
        let b = (255.99 * self.b) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}
