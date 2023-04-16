mod camera;
mod hittable;
mod random;
mod ray;
mod sphere;
mod vec3;
mod world;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::random::{random_f32, random_f32_range};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};
use crate::world::World;
use std::fs::File;
use std::io::{BufWriter, Error, Write};

const IMAGE_WIDTH: f32 = 400.0;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: f32 = IMAGE_WIDTH / ASPECT_RATIO;
const SAMPLES_PER_PIXEL: f32 = 100.0;

fn ray_color(r: Ray, world: &World) -> Color {
    if let Some(hit_record) = world.hit(r, 0.0, std::f32::INFINITY) {
        return 0.5 * (hit_record.normal + Color::diagonal(1.0));
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::diagonal(1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Error> {
    let camera = Camera::new();

    let mut world = World::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Render here

    let f = File::create("./image.ppm").expect("Unable to create file.");
    let mut f = BufWriter::new(f);
    f.write(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes())?;

    for j in (0..IMAGE_HEIGHT as i32).rev() {
        if j % 50 == 0 {
            println!("{j} scanlines remaining");
        }
        for i in 0..(IMAGE_WIDTH as i32) {
            let mut pixel_color = Color::default();
            for _sample in 0..SAMPLES_PER_PIXEL as i32 {
                let u = (i as f32 + random_f32_range(-1.0..=1.0)) / (IMAGE_WIDTH - 1.0);
                let v = (j as f32 + random_f32_range(-1.0..=1.0)) / (IMAGE_HEIGHT - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world)
            }

            f.write(pixel_color.color(SAMPLES_PER_PIXEL).as_bytes())?;
        }
    }

    Ok(())
}
