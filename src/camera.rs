use rand::prelude::random;
use std::fs::File;
use std::io::Write;

use crate::color::*;
use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub image_height: u32,
    pub camera_center: Vec3,
    pub pixel00_location: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let (image_height, camera_center, pixel00_location, pixel_delta_u, pixel_delta_v) =
            Camera::initialize(aspect_ratio, image_width);
        Self {
            aspect_ratio,
            image_width,
            image_height,
            camera_center,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
        }
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random::<f32>() - 0.5, random::<f32>() - 0.5, 0.)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a ray starting from the camera and pointing to a randomly
        // sampled location in the i,j pixel
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_location
            + (i as f32 + offset.x) * self.pixel_delta_u
            + (j as f32 + offset.y) * self.pixel_delta_v;

        Ray::new(self.camera_center, pixel_sample - self.camera_center)
    }

    fn ray_color(ray: Ray, depth: u32, world: &HittableList) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0., 0., 0.);
        }
        let mut hit_data = HitData::default();
        if world.hit(ray, Interval::new(0.001, f32::INFINITY), &mut hit_data) {
            let mut attenuation = Vec3::default();
            let mut scattered = Ray::default();
            if let Some(material) = hit_data.material {
                if material.scatter(ray, hit_data, &mut attenuation, &mut scattered) {
                    return attenuation * Self::ray_color(scattered, depth - 1, world);
                }
            }
            return Vec3::new(0., 0., 0.);
        }

        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.);
        (1. - a) * Vec3::new(1., 1., 1.) + a * Vec3::new(0.5, 0.7, 1.)
    }

    fn initialize(aspect_ratio: f32, image_width: u32) -> (u32, Vec3, Vec3, Vec3, Vec3) {
        // Initialize camera characteristics
        let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

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

        (
            image_height,
            camera_center,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
        )
    }

    pub fn render(&self, world: &HittableList) {
        // File creation
        if let Ok(mut file) = File::create("image.ppm") {
            let _ = file.write_all(
                format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes(),
            );

            for j in 0..self.image_height {
                for i in 0..self.image_width {
                    let mut pixel_color = Vec3::new(0., 0., 0.);
                    for _sample in 0..self.samples_per_pixel {
                        let ray = self.get_ray(i, j);
                        pixel_color += Self::ray_color(ray, self.max_depth, world)
                    }

                    let _ = file.write_all(&write_color(
                        (1. / self.samples_per_pixel as f32) * pixel_color,
                    ));
                }
            }
        }
    }
}
