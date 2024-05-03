use rand::prelude::random;
use std::fs::File;
use std::io::Write;

use crate::color::*;
use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::utilities::degrees_to_radians;
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
    // Parameters for positionable camera with variable field of view
    pub vertical_fov: f32,
    pub look_from: Vec3,
    pub look_to: Vec3,
    // Up vector chosen prior to projection/normalization to form
    // orthonormal camera coordinate system
    pub vec_up: Vec3,
    // Parameters both supplied and derived for defocus blur
    pub defocus_angle: f32,
    pub focus_distance: f32,
    pub defocus_disc_u: Vec3,
    pub defocus_disc_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vertical_fov: f32,
        look_from: Vec3,
        look_to: Vec3,
        vec_up: Vec3,
        defocus_angle: f32, // Full angle of cone with tip at focus point and base camera "lens"
        focus_distance: f32, // Distance from camera "lens" center and focus plane
    ) -> Self {
        let (
            image_height,
            camera_center,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disc_u,
            defocus_disc_v,
        ) = Camera::initialize(
            aspect_ratio,
            image_width,
            vertical_fov,
            look_from,
            look_to,
            vec_up,
            defocus_angle,
            focus_distance,
        );
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
            vertical_fov,
            look_from,
            look_to,
            vec_up,
            defocus_angle,
            focus_distance,
            defocus_disc_u,
            defocus_disc_v,
        }
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random::<f32>() - 0.5, random::<f32>() - 0.5, 0.)
    }

    fn defocus_disc_sample(&self) -> Vec3 {
        // Returns random point inside camera defocus disc
        let point = Vec3::random_in_unit_disc();
        self.camera_center + point.x * self.defocus_disc_u + point.y * self.defocus_disc_v
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a ray starting from the camera defocus disc and pointing to a randomly
        // sampled location in the i,j pixel
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_location
            + (i as f32 + offset.x) * self.pixel_delta_u
            + (j as f32 + offset.y) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0. {
            self.camera_center
        } else {
            self.defocus_disc_sample()
        };
        Ray::new(ray_origin, pixel_sample - ray_origin)
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

    fn initialize(
        aspect_ratio: f32,
        image_width: u32,
        vertical_fov: f32,
        look_from: Vec3,
        look_to: Vec3,
        vec_up: Vec3,
        defocus_angle: f32,
        focus_distance: f32,
    ) -> (u32, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3) {
        // Initialize camera characteristics
        let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let camera_center = look_from;
        let viewport_height: f32 =
            2.0 * (degrees_to_radians(vertical_fov) / 2.).tan() * focus_distance;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

        // Define camera orthonormal coordinate system
        let w = (look_from - look_to).unit();
        let u = Vec3::cross(vec_up, w).unit();
        let v = Vec3::cross(w, u);

        // Calculate vectors across vertical and horizontal viewport edges of
        // the camera
        let viewport_u = viewport_width * u; // Horizontal edge vector
        let viewport_v = -viewport_height * v; // Vertical edge vector

        // Calculate the horizontal and vertical pixel-to-pixel vectors
        let pixel_delta_u = (1. / image_width as f32) * viewport_u;
        let pixel_delta_v = (1. / image_height as f32) * viewport_v;

        // Calculate location of upper-left pixel in the image
        let viewport_upper_left =
            camera_center - focus_distance * w - 0.5 * viewport_u - 0.5 * viewport_v;
        let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate camera defocus disc vectors
        let defocus_radius = focus_distance * (degrees_to_radians(defocus_angle / 2.)).tan();
        let defocus_disc_u = defocus_radius * u;
        let defocus_disc_v = defocus_radius * v;

        (
            image_height,
            camera_center,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disc_u,
            defocus_disc_v,
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
