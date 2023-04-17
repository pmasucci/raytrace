mod camera;
mod hittable;
mod random;
mod ray;
mod scatterable;
mod sphere;
mod vec3;
mod world;

use scatterable::{Lambertian, Metal};

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::random::random_f32;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3};
use crate::world::World;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

const IMAGE_WIDTH: f32 = 3840.0;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: f32 = IMAGE_WIDTH / ASPECT_RATIO;
const SAMPLES_PER_PIXEL: f32 = 100.0;
const MAX_DEPTH: i32 = 50;

fn ray_color(r: Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color::diagonal(0.0);
    }
    if let Some(hit_record) = world.hit(r, 0.001, std::f32::INFINITY) {
        return match hit_record.material.scatter(r, &hit_record) {
            Some((scattered, attenuation)) => attenuation * ray_color(scattered, world, depth - 1),
            None => Color::new(0.0, 0.0, 0.0),
        };
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // filling in sky color
    (1.0 - t) * Color::diagonal(1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Error> {
    let camera = Camera::new();
    let mut image = vec![0; (IMAGE_WIDTH * IMAGE_HEIGHT * 3.0) as usize];
    let bands: Vec<(usize, &mut [u8])> = image
        .chunks_mut((IMAGE_WIDTH * 3.0) as usize)
        .enumerate()
        .collect();

    let world = World::default();

    // Render here

    let f = File::create("./image.ppm").expect("Unable to create file.");
    let mut f = BufWriter::new(f);
    f.write(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes())?;
    static ELAPSED: AtomicUsize = AtomicUsize::new(0);
    let start = Instant::now();
    bands.into_par_iter().for_each(|(i, band)| {
        for x in 0..IMAGE_WIDTH as usize {
            let mut pixel_color = Color::default();
            for _s in 0..SAMPLES_PER_PIXEL as i32 {
                let u = (x as f32 + random_f32()) / (IMAGE_WIDTH - 1.0);
                let v = (IMAGE_HEIGHT - (i as f32 + random_f32())) / (IMAGE_HEIGHT - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            let (r, g, b) = pixel_color.color(SAMPLES_PER_PIXEL);
            band[x * 3] = r;
            band[x * 3 + 1] = g;
            band[x * 3 + 2] = b;
        }
        let elapsed_count = ELAPSED.fetch_add(1, Ordering::SeqCst) + 1;

        if elapsed_count % 50 == 0 {
            println!("{}/{}", elapsed_count, IMAGE_HEIGHT);
        }
    });

    println!("Frame time: {}ms", start.elapsed().as_millis());

    image.chunks(3).for_each(|color| {
        let _ = f.write(format!("{} {} {}\n", color[0], color[1], color[2]).as_bytes());
    });

    Ok(())
}
