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
use crate::material::{Dielectric, Lambertian, Metal};
use crate::number::random_f64;
use crate::pixel::Pixel;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = std::sync::Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(std::sync::Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_f64(None);
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(None),
                0.2,
                b as f64 + 0.9 * random_f64(None),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random(None) * Color::random(None);
                    let material = std::sync::Arc::new(Lambertian::new(albedo));
                    world.add(std::sync::Arc::new(Sphere::new(center, 0.2, material)));
                } else if choose_material < 0.95 {
                    let albedo = Color::random(Some((0.5, 1.0)));
                    let fuzz = random_f64(Some((0.0, 0.5)));
                    let material = std::sync::Arc::new(Metal::new(albedo, fuzz));
                    world.add(std::sync::Arc::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = std::sync::Arc::new(Dielectric::new(1.5));
                    world.add(std::sync::Arc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material_1 = std::sync::Arc::new(Dielectric::new(1.5));
    world.add(std::sync::Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = std::sync::Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(std::sync::Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = std::sync::Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(std::sync::Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 1920;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel: usize = 500;
    let max_depth: usize = 50;

    // camera
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );
    let camera = std::sync::Arc::new(camera);

    // world
    let world = random_scene();

    let world: std::sync::Arc<(dyn Hittable + 'static + Send)> = std::sync::Arc::new(world);

    // processor
    let mut functions = Vec::new();
    for _ in 0..num_cpus::get() {
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
