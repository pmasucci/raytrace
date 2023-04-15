mod ray;
mod vec3;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use std::fs::File;
use std::io::{BufWriter, Error, Write};

const IMAGE_WIDTH: f32 = 400.0;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: f32 = IMAGE_WIDTH / ASPECT_RATIO;

fn ray_color(r: Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::diagonal(1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c; // b^2 - 4ac from the quadratic equation
    discriminant > 0.0 // a 0 for the discriminant means there is at least one intersection between this ray and the sphere
}

fn main() -> Result<(), Error> {
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

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

            let pixel_color = ray_color(r);

            f.write(pixel_color.color().as_bytes())?;
        }
    }

    Ok(())
}
