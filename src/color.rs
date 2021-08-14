use crate::vec3::Vec3;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
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

impl std::ops::Add for &Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        &rhs * self
    }
}

impl std::ops::Add<&Vec3> for &Color {
    type Output = Color;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            r: self.r + rhs.x(),
            g: self.g + rhs.y(),
            b: self.b + rhs.z(),
        }
    }
}

impl std::ops::Add<Color> for &Vec3 {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        &rhs + self
    }
}
