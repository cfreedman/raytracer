use core::f32;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::color::*;
use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::utilities::{degrees_to_radians, random_num};
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
    pub background: Option<Vec3>, // Color for background
}

impl Default for Camera {
    fn default() -> Self {
        Self::init().build()
    }
}

impl Camera {
    pub fn init() -> CameraBuilder {
        CameraBuilder::default()
    }

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
        background: Option<Vec3>,
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
            background,
        }
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_num() - 0.5, random_num() - 0.5, 0.)
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
        Ray::new(ray_origin, pixel_sample - ray_origin, random_num())
    }

    fn ray_color(&self, ray: Ray, depth: u32, world: &HittableList) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0., 0., 0.);
        }

        let mut hit_data = HitData::default();

        if !world.hit(ray, Interval::new(0.001, f32::INFINITY), &mut hit_data) {
            return if self.background.is_some() {
                self.background.unwrap()
            } else {
                let unit_direction = ray.direction.unit();
                let a = 0.5 * (unit_direction.y + 1.);
                (1. - a) * Vec3::new(1., 1., 1.) + a * Vec3::new(0.5, 0.7, 1.)
            };
        }

        let mut attenuation = Vec3::default();
        let mut scattered = Ray::default();
        if let Some(material) = hit_data.clone().material {
            let emitted_color = material.emit(hit_data.point, hit_data.u, hit_data.v);
            if !material.scatter(ray, &mut hit_data, &mut attenuation, &mut scattered) {
                return emitted_color;
            }

            return emitted_color + attenuation * self.ray_color(scattered, depth - 1, world);
        }

        Vec3::ZERO
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
                        pixel_color += self.ray_color(ray, self.max_depth, world)
                    }

                    let _ = file.write_all(&write_color(
                        (1. / self.samples_per_pixel as f32) * pixel_color,
                    ));

                    eprintln!("Write pixel ({i},{j}) successfully")
                }
            }
        }
    }

    pub fn render_to_disc(&self, filename: &str, world: &HittableList) -> io::Result<()> {
        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .progress_count((self.image_height * self.image_width) as u64)
            .map(|(y, x)| {
                let multisampled_color = (0..self.samples_per_pixel)
                    .into_iter()
                    .map(|_| {
                        let ray = self.get_ray(x, y);
                        self.ray_color(ray, self.max_depth, world)
                    })
                    .sum::<Vec3>();
                let multisampled_color = (1. / self.samples_per_pixel as f32) * multisampled_color;

                let final_color = 256.
                    * (Vec3::new(
                        linear_to_gamma(multisampled_color.x),
                        linear_to_gamma(multisampled_color.y),
                        linear_to_gamma(multisampled_color.z),
                    )
                    .clamp(Vec3::ZERO, Vec3::splat(0.999)));
                format!(
                    "{} {} {}\n",
                    final_color.x as u8, final_color.y as u8, final_color.z as u8
                )
            })
            .collect::<Vec<String>>()
            .join("");

        fs::write(
            format!("output/{filename}.ppm"),
            format!(
                "P3\n{} {}\n255\n{}",
                self.image_width, self.image_height, pixels
            ),
        )
    }
}

pub struct CameraBuilder {
    aspect_ratio: f32,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    vertical_fov: f32,
    look_from: Vec3,
    look_to: Vec3,
    vec_up: Vec3,
    defocus_angle: f32,
    focus_distance: f32,
    background: Option<Vec3>,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 16. / 9.,
            image_width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            vertical_fov: 20.,
            look_from: Vec3::new(0., 0., -10.),
            look_to: Vec3::ZERO,
            vec_up: Vec3::new(0., 1., 0.),
            defocus_angle: 0.,
            focus_distance: 10.,
            background: None,
        }
    }
}

impl CameraBuilder {
    pub fn image_width(mut self, width: u32) -> Self {
        self.image_width = width;
        self
    }

    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = ratio;
        self
    }

    pub fn samples_per_pixel(mut self, samples: u32) -> Self {
        self.samples_per_pixel = samples;
        self
    }

    pub fn max_depth(mut self, depth: u32) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn vertical_fov(mut self, fov: f32) -> Self {
        self.vertical_fov = fov;
        self
    }

    pub fn look_from(mut self, vec: Vec3) -> Self {
        self.look_from = vec;
        self
    }

    pub fn look_to(mut self, vec: Vec3) -> Self {
        self.look_to = vec;
        self
    }

    pub fn vec_up(mut self, vec: Vec3) -> Self {
        self.vec_up = vec;
        self
    }

    pub fn defocus_angle(mut self, angle: f32) -> Self {
        self.defocus_angle = angle;
        self
    }

    pub fn focus_distance(mut self, distance: f32) -> Self {
        self.focus_distance = distance;
        self
    }

    pub fn background(mut self, background: Vec3) -> Self {
        self.background = Some(background);
        self
    }

    pub fn build(self) -> Camera {
        // Initialize camera characteristics
        let image_height: u32 = (self.image_width as f32 / self.aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let camera_center = self.look_from;
        let viewport_height: f32 =
            2.0 * (degrees_to_radians(self.vertical_fov) / 2.).tan() * self.focus_distance;
        let viewport_width: f32 = viewport_height * (self.image_width as f32 / image_height as f32);

        // Define camera orthonormal coordinate system
        let w = (self.look_from - self.look_to).unit();
        let u = Vec3::cross(self.vec_up, w).unit();
        let v = Vec3::cross(w, u);

        // Calculate vectors across vertical and horizontal viewport edges of
        // the camera
        let viewport_u = viewport_width * u; // Horizontal edge vector
        let viewport_v = -viewport_height * v; // Vertical edge vector

        // Calculate the horizontal and vertical pixel-to-pixel vectors
        let pixel_delta_u = (1. / self.image_width as f32) * viewport_u;
        let pixel_delta_v = (1. / image_height as f32) * viewport_v;

        // Calculate location of upper-left pixel in the image
        let viewport_upper_left =
            camera_center - self.focus_distance * w - 0.5 * viewport_u - 0.5 * viewport_v;
        let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate camera defocus disc vectors
        let defocus_radius =
            self.focus_distance * (degrees_to_radians(self.defocus_angle / 2.)).tan();
        let defocus_disc_u = defocus_radius * u;
        let defocus_disc_v = defocus_radius * v;

        Camera {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            image_height,
            camera_center,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            vertical_fov: self.vertical_fov,
            look_from: self.look_from,
            look_to: self.look_to,
            vec_up: self.vec_up,
            defocus_angle: self.defocus_angle,
            focus_distance: self.focus_distance,
            defocus_disc_u,
            defocus_disc_v,
            background: self.background,
        }
    }
}
