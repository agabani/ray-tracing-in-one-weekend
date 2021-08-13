use crate::color::Color;
use crate::Pixel;

pub struct Buffer {
    buffer: Vec<Color>,
    height: usize,
    width: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = Vec::with_capacity(height * width);

        for _ in 0..height {
            for _ in 0..width {
                buffer.push(Color::new(0.0, 0.0, 0.0));
            }
        }

        Self {
            buffer,
            height,
            width,
        }
    }

    pub fn get(&self, pixel: &Pixel) -> &Color {
        &self.buffer[self.width * pixel.j + pixel.i]
    }

    pub fn set(&mut self, pixel: &Pixel, color: Color) {
        self.buffer[self.width * pixel.j + pixel.i] = color;
    }
}
