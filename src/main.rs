use std::fs::File;
use std::io::prelude::*;

mod color;
mod ray;
mod vec3;
use vec3::*;

pub fn ray_color() -> Vec3 {
    return Vec3::ZERO;
}

fn main() {
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

    let pixel_delta_u = constant_mult(&viewport_u, &(1. / image_width as f32));
    let pixel_delta_v = constant_mult(&viewport_v, &(1. / image_height as f32));

    let viewport_upper_left = camera_center
        - Vec3::new(0., 0., focal_length)
        - constant_mult(&viewport_u, &0.5)
        - constant_mult(&viewport_v, &0.5);
    let pixel00_location =
        viewport_upper_left + constant_mult(&(pixel_delta_u + pixel_delta_v), &0.5);

    /* let mut file = File::create("image.ppm")?;

    let image_width = 256;
    let image_height = 256;

    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes());

    for i in 0..image_height {
        println!("{} scan lines remaining", image_height - i);
        for j in 0..image_width {
            let r: f32 = j as f32 / ((image_width - 1) as f32);
            let g: f32 = i as f32 / ((image_height - 1) as f32);
            let b: f32 = 0.0;

            let color = vec3::Vec3 { x: r, y: g, z: b };

            file.write_all(&color::write_color(&color));
        }
    }
    println!("Done!");
    Ok(()) */
}
