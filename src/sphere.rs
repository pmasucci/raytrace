use std::rc::Rc;
use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::scatterable::Scatterable;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Arc<Box<dyn Scatterable + Sync + Send>>,
}

impl Sphere {
    pub fn new(
        center: Point3,
        radius: f32,
        material: Arc<Box<dyn Scatterable + Sync + Send>>,
    ) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = r.at(root);
        let outward_normal = (point - self.center) / self.radius;

        let hit = HitRecord::new(point, root, r, outward_normal, self.material.clone());

        Some(hit)
    }
}
