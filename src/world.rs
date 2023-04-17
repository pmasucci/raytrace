use crate::{
    hittable::{HitRecord, Hittable},
    scatterable::{Lambertian, Metal},
    sphere::Sphere,
    vec3::{Color, Point3},
};
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

impl Default for World {
    fn default() -> Self {
        let mut world = World::new();
        let material_ground = Lambertian {
            albedo: Color::new(0.8, 0.8, 0.0),
        };
        let material_center = Lambertian {
            albedo: Color::new(0.7, 0.3, 0.3),
        };
        let material_left = Metal {
            albedo: Color::new(0.8, 0.8, 0.8),
        };
        let material_right = Metal {
            albedo: Color::new(0.8, 0.6, 0.2),
        };

        world.add(Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Arc::new(Box::new(material_ground)),
        )));

        world.add(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Box::new(material_center)),
        )));

        world.add(Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Arc::new(Box::new(material_left)),
        )));

        world.add(Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Arc::new(Box::new(material_right)),
        )));
        world
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
