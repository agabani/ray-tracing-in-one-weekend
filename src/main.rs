mod buffer;
mod color;

use crate::buffer::Buffer;
use crate::color::Color;

pub struct Pixel {
    i: usize,
    j: usize,
}

impl Pixel {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }
}

pub struct ComputeResult {
    id: usize,
    pixel: Pixel,
    color: Color,
}

impl ComputeResult {
    pub fn new(id: usize, pixel: Pixel, color: Color) -> Self {
        Self { id, pixel, color }
    }
}

fn main() {
    // configuration
    let image_width: usize = 256;
    let image_height: usize = 256;

    // processor
    let (orchestration_tx, orchestration_rx) = std::sync::mpsc::channel();
    let mut computes_tx = std::collections::HashMap::<usize, std::sync::mpsc::Sender<Pixel>>::new();

    for compute_id in 0..16 {
        let orchestration_tx = orchestration_tx.clone();
        let (compute_tx, compute_rx) = std::sync::mpsc::channel();
        computes_tx.insert(compute_id, compute_tx);

        std::thread::spawn(move || {
            for pixel in compute_rx {
                let r = (pixel.i as f64) / (image_width as f64 - 1.0);
                let g = (pixel.j as f64) / (image_height as f64 - 1.0);
                let b = 0.25;

                let color = Color::new(r, g, b);

                orchestration_tx
                    .send(ComputeResult::new(compute_id, pixel, color))
                    .unwrap();
            }
        });
    }

    // orchestrator
    let mut buffer = Buffer::new(image_width, image_height);

    let mut processed = 0;
    let total = image_height * image_width;

    let mut jobs = Vec::with_capacity(total);
    for j in 0..image_height {
        for i in 0..image_width {
            jobs.push(Pixel::new(i, j));
        }
    }

    for computes_tx in computes_tx.values() {
        if let Some(pixel) = jobs.pop() {
            computes_tx.send(pixel).unwrap();
        }
    }

    for result in orchestration_rx {
        processed += 1;

        buffer.set(&result.pixel, result.color);

        if processed < total {
            let compute_tx = &computes_tx.get(&result.id).unwrap();
            if let Some(pixel) = jobs.pop() {
                compute_tx.send(pixel).unwrap();
            }
        } else {
            break;
        }
    }

    // save buffer
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            println!("{}", buffer.get(&Pixel::new(i, j)))
        }
    }
}
