use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_unit_vector, Color},
};

pub trait Scatterable {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.point, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray_in.direction.unit_vector().reflect(hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);
        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
