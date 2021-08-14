use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    front_face: bool,
    normal: Vec3,
    point: Vec3,
    time: f64,
}

impl HitRecord {
    fn new(ray: &Ray, outward_normal: Vec3, point: Vec3, time: f64) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            front_face,
            normal,
            point,
            time,
        }
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn point(&self) -> &Vec3 {
        &self.point
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let square_root_of_discriminant = discriminant.sqrt();

        let root = (-half_b - square_root_of_discriminant) / a;
        if root < time_min || time_max < root {
            let root = (-half_b - square_root_of_discriminant) / a;
            if root < time_min || time_max < root {
                return None;
            }
        }

        let time = root;
        let point = ray.at(time);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord::new(ray, normal, point, time))
    }
}
