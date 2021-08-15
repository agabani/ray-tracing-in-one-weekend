mod buffer;
mod camera;
mod color;
mod compute;
mod hittable;
mod hittable_list;
mod material;
mod number;
mod pixel;
mod ray;
mod vec3;

use crate::buffer::Buffer;
use crate::camera::Camera;
use crate::color::Color;
use crate::compute::Compute;
use crate::hittable::{Hittable, Sphere};
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal};
use crate::number::random_f64;
use crate::pixel::Pixel;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;

    // camera
    let camera = Camera::new();
    let camera = std::sync::Arc::new(camera);

    // world
    let mut world = HittableList::new();

    let material_ground = std::sync::Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = std::sync::Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = std::sync::Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = std::sync::Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(std::sync::Arc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(std::sync::Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(std::sync::Arc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(std::sync::Arc::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let world: std::sync::Arc<(dyn Hittable + 'static + Send)> = std::sync::Arc::new(world);

    // processor
    let mut functions = Vec::new();
    for _ in 0..16 {
        let camera = camera.clone();
        let world = world.clone();
        functions.push(move |pixel: &Pixel| {
            fn ray_color(
                ray: &Ray,
                world: &std::sync::Arc<(dyn Hittable + 'static + Send)>,
                depth: usize,
            ) -> Color {
                if depth == 0 {
                    return Color::new(0.0, 0.0, 0.0);
                }

                if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
                    return if let Some((attenuation, scattered)) =
                        hit_record.material().scatter(ray, &hit_record)
                    {
                        attenuation * ray_color(&scattered, world, depth - 1)
                    } else {
                        Color::new(0.0, 0.0, 0.0)
                    };
                }
                let unit_direction = ray.direction().unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }

            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (pixel.i() as f64 + random_f64(None)) / (image_width as f64 - 1.0);
                let v = (pixel.j() as f64 + random_f64(None)) / (image_height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                color = color + ray_color(&ray, &world, max_depth);
            }
            color
        });
    }

    let (compute, receiver) = Compute::new(functions);

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
            println!(
                "{}",
                buffer
                    .get(&Pixel::new(i, j))
                    .sampled(samples_per_pixel)
                    .gamma()
            )
        }
    }
}
