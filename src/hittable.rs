use crate::ray::*;
use crate::vec3::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct HitData {
    pub hit_along_ray: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitData {
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.direction, outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -1. * outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_tmin: f32, ray_tmax: f32, hit_data: &mut HitData) -> bool;
}

#[derive(Default)]
pub struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.0.push(object)
    }

    pub fn clear(&mut self) {
        self.clear()
    }

    pub fn hit(&self, ray: Ray, ray_tmin: f32, ray_tmax: f32, hit_data: &mut HitData) -> bool {
        let mut temp_hit_data = HitData::default();
        let mut hit_anything = false;
        let mut closest_hit = ray_tmax;

        for object in self.0.iter() {
            if object.hit(ray, ray_tmin, closest_hit, &mut temp_hit_data) {
                hit_anything = true;
                closest_hit = temp_hit_data.hit_along_ray;
                *hit_data = temp_hit_data;
            }
        }

        hit_anything
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_tmin: f32, ray_tmax: f32, hit_data: &mut HitData) -> bool {
        let origin_gap = self.center - ray.origin;
        // Quadratic constants for solving the ray intersection equation
        let a = ray.direction.length_squared();
        // Use scaled parameter for symplicity h = b / -2
        let h = dot(ray.direction, origin_gap);
        let c = origin_gap.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return false;
        }

        let mut ray_hit = (h - discriminant.sqrt()) / a;
        if (ray_hit <= ray_tmin) || (ray_hit >= ray_tmax) {
            ray_hit = (h + discriminant.sqrt()) / a;
            if (ray_hit <= ray_tmin) || (ray_hit >= ray_tmax) {
                return false;
            }
        }

        hit_data.hit_along_ray = ray_hit;
        hit_data.point = ray.at(ray_hit);
        let outward_normal = (1. / self.radius) * (hit_data.point - self.center);
        hit_data.set_face_normal(ray, outward_normal);

        true
    }
}
