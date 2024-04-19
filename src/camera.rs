use std::io;
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
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        Self {
            aspect_ratio,
            image_width,
        }
    }

    fn ray_color(ray: Ray, world: &HittableList) -> Vec3 {
        let mut hit_data = HitData::default();
        if world.hit(ray, Interval::new(0., f32::INFINITY), &mut hit_data) {
            return 0.5 * (hit_data.normal + Vec3::new(1., 1., 1.));
        }

        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.);
        (1. - a) * Vec3::new(1., 1., 1.) + a * Vec3::new(0.5, 0.7, 1.)
    }

    fn initialize(&self) -> (u32, Vec3, Vec3, Vec3, Vec3) {
        // Initialize camera characteristics
        let image_height: u32 = (self.image_width as f32 / self.aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let focal_length = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (self.image_width as f32 / image_height as f32);
        let camera_center = Vec3::ZERO;

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        let pixel_delta_u = (1. / self.image_width as f32) * viewport_u;
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
        let (image_height, camera_center, pixel00_location, pixel_delta_u, pixel_delta_v) =
            self.initialize();

        io::stdout()
            .write_all(format!("P3\n{} {}\n255\n", self.image_width, image_height).as_bytes());

        for j in 0..image_height {
            for i in 0..self.image_width {
                let pixel_center =
                    pixel00_location + (i as f32) * pixel_delta_u + (j as f32) * pixel_delta_v;
                let ray_direction = pixel_center - camera_center;
                let r = Ray {
                    origin: camera_center,
                    direction: ray_direction,
                };

                let pixel_color = Self::ray_color(r, &world);
                io::stdout().write_all(&write_color(&pixel_color));
            }
        }
    }
}
