use crate::ray::Ray;
use crate::scatterable::Scatterable;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<Box<dyn Scatterable + Sync + Send>>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        t: f32,
        ray: Ray,
        outward_normal: Vec3,
        material: Arc<Box<dyn Scatterable + Sync + Send>>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
