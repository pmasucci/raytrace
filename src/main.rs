mod hittable;
mod ray;
mod sphere;
mod vec3;
mod world;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};
use crate::world::World;
use std::fs::File;
use std::io::{BufWriter, Error, Write};

const IMAGE_WIDTH: f32 = 400.0;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: f32 = IMAGE_WIDTH / ASPECT_RATIO;

fn ray_color(r: Ray, world: &World) -> Color {
    if let Some(hit_record) = world.hit(r, 0.0, std::f32::INFINITY) {
        return 0.5 * (hit_record.normal + Color::diagonal(1.0));
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::diagonal(1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Error> {
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let mut world = World::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render here

    let f = File::create("./image.ppm").expect("Unable to create file.");
    let mut f = BufWriter::new(f);
    f.write(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes())?;

    for j in (0..IMAGE_HEIGHT as i32).rev() {
        if j % 50 == 0 {
            println!("{j} scanlines remaining");
        }
        for i in 0..IMAGE_WIDTH as i32 {
            let u = i as f32 / (IMAGE_WIDTH - 1.0);
            let v = j as f32 / (IMAGE_HEIGHT - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(r, &world);

            f.write(pixel_color.color().as_bytes())?;
        }
    }

    Ok(())
}
