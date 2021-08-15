use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal() + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal().clone();
        }

        let scattered = Ray::new(hit_record.point().clone(), scatter_direction);
        Some((self.albedo.clone(), scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction().unit_vector().reflect(hit_record.normal());
        let scattered = Ray::new(
            hit_record.point().clone(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if scattered.direction().dot(hit_record.normal()) > 0.0 {
            Some((self.albedo.clone(), scattered))
        } else {
            None
        }
    }
}