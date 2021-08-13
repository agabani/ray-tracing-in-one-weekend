mod buffer;
mod color;
mod compute;
mod pixel;

use crate::buffer::Buffer;
use crate::color::Color;
use crate::compute::Compute;
use crate::pixel::Pixel;

fn main() {
    // configuration
    let image_width: usize = 256;
    let image_height: usize = 256;

    // processor
    let (orchestration_tx, orchestration_rx) = std::sync::mpsc::channel();
    // processor
    let compute = Compute::new(16, orchestration_tx.clone(), move |pixel| {
        let r = (pixel.i() as f64) / (image_width as f64 - 1.0);
        let g = (pixel.j() as f64) / (image_height as f64 - 1.0);
        let b = 0.25;

        Color::new(r, g, b)
    });

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

    for computes_tx in compute.computes_tx().values() {
        if let Some(pixel) = jobs.pop() {
            computes_tx.send(pixel).unwrap();
        }
    }

    for result in orchestration_rx {
        processed += 1;

        buffer.set(&result.pixel(), result.color().clone());

        if processed < total {
            let compute_tx = &compute.computes_tx().get(&result.id()).unwrap();
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
