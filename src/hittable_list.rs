use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<std::sync::Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: std::sync::Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = time_max;
        let mut closest_hit_record = None;

        for object in &self.objects {
            let hit_record = object.hit(ray, time_min, closest_so_far);
            if let Some(hit_record) = hit_record {
                closest_so_far = hit_record.time();
                closest_hit_record = Some(hit_record);
            }
        }

        closest_hit_record
    }
}
