mod buffer;
mod color;
mod compute;
mod hittable;
mod pixel;
mod ray;
mod vec3;
mod hittable_list;

use crate::buffer::Buffer;
use crate::color::Color;
use crate::compute::Compute;
use crate::pixel::Pixel;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.length_squared() - radius.powi(2);
    let d = half_b.powi(2) - a * c;
    if d < 0.0 {
        -1.0
    } else {
        (-half_b - d.sqrt()) / a
    }
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // processor
    let (compute, receiver) = Compute::new(16, move |pixel: &Pixel| {
        fn ray_color(ray: &Ray) -> Color {
            let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
            if t > 0.0 {
                let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
                return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
            }
            let unit_direction = ray.direction().unit();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }

        let u = (pixel.i() as f64) / (image_width as f64 - 1.0);
        let v = (pixel.j() as f64) / (image_height as f64 - 1.0);

        let ray = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );

        ray_color(&ray)
    });

    // orchestrator
    let mut jobs = Vec::with_capacity(image_height * image_width);
    for j in 0..image_height {
        for i in 0..image_width {
            jobs.push(Pixel::new(i, j));
        }
    }

    let buffer = compute.compute_all(
        &receiver,
        jobs,
        |mut buffer, pixel, color| {
            buffer.set(pixel, color);
            buffer
        },
        Buffer::new(image_width, image_height),
    );

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
