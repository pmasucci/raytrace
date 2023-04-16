use crate::hittable::{HitRecord, Hittable};
use std::sync::Arc;

pub struct World {
    objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl World {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Sync + Send>) {
        self.objects.push(object);
    }

    pub fn new() -> Self {
        Self { objects: vec![] }
    }
}

impl Hittable for World {
    fn hit(&self, r: crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(temp_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                hit_record = Some(temp_record);
            }
        }
        hit_record
    }
}
