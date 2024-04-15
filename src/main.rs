use std::fs::File;
use std::io::{prelude::*, Error};

mod color;
mod ray;
use ray::*;
mod vec3;
use vec3::*;

pub fn ray_color(ray: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0., 0., -1.), 0.5, ray) {
        return Vec3::new(1., 0., 0.);
    }
    let unit_direction = ray.direction;
    let a = 0.5 * (unit_direction.y + 1.);
    (1. - a) * Vec3::new(1., 1., 1.) + a * Vec3::new(0.5, 0.7, 1.)
}

pub fn hit_sphere(center: Vec3, radius: f32, ray: Ray) -> bool {
    let origin_gap = center - ray.origin;
    // Quadratic constants for solving the ray intersection equation
    let a = dot(ray.direction, ray.direction);
    let b = -2. * dot(ray.direction, origin_gap);
    let c = dot(origin_gap, origin_gap) - radius * radius;
    b * b - 4. * a * c >= 0.
}

fn main() -> Result<(), Error> {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    // Camera
    let focal_length = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::ZERO;

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    let pixel_delta_u = (1. / image_width as f32) * viewport_u;
    let pixel_delta_v = (1. / image_height as f32) * viewport_v;

    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., focal_length) - 0.5 * viewport_u - 0.5 * viewport_v;
    let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut file = File::create("image.ppm")?;

    // Render

    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes());

    for j in 0..image_height {
        println!("{} scan lines remaining", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_location + (i as f32) * pixel_delta_u + (j as f32) * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let r = Ray {
                origin: pixel_center,
                direction: ray_direction,
            };

            let pixel_color = ray_color(r);
            file.write_all(&color::write_color(&pixel_color));
        }
    }
    println!("Done!");
    Ok(())
}
