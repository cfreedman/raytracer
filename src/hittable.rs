use crate::aabb::Aabb;
use crate::interval::*;
use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;

pub struct HitData {
    pub hit_along_ray: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Option<Material>,
}

impl Default for HitData {
    fn default() -> Self {
        Self {
            hit_along_ray: 0.,
            point: Vec3::default(),
            normal: Vec3::default(),
            front_face: false,
            material: None,
        }
    }
}

impl HitData {
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction, outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -1. * outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, interval: Interval, hit_data: &mut HitData) -> bool;

    fn bounding_box(&self) -> Aabb;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = Aabb::new_from_boxes(self.bbox, object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = Aabb::default();
    }

    pub fn hit(&self, ray: Ray, interval: Interval, hit_data: &mut HitData) -> bool {
        let mut hit_anything = false;
        let mut closest_hit = interval.max;

        for object in self.objects.iter() {
            // Define temp_hit_data in each loop iteration to avoid its moving
            // across loop counts
            let mut temp_hit_data = HitData::default();
            if object.hit(
                ray,
                Interval::new(interval.min, closest_hit),
                &mut temp_hit_data,
            ) {
                hit_anything = true;
                closest_hit = temp_hit_data.hit_along_ray;
                *hit_data = temp_hit_data;
            }
        }

        hit_anything
    }
}

pub struct Sphere {
    pub center_0: Vec3,
    pub center_1: Vec3,
    pub radius: f32,
    pub material: Material,
    pub bbox: Aabb,
}

impl Sphere {
    pub fn new(center_0: Vec3, center_1: Vec3, radius: f32, material: Material) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox_0 = Aabb::new_from_points(center_0 - rvec, center_0 + rvec);
        let bbox_1 = Aabb::new_from_points(center_1 - rvec, center_1 + rvec);
        let bbox = Aabb::new_from_boxes(bbox_0, bbox_1);

        Self {
            center_0,
            center_1,
            radius,
            material,
            bbox,
        }
    }

    pub fn sphere_center(&self, time: f32) -> Vec3 {
        let center_vec = self.center_1 - self.center_0;
        self.center_0 + time * center_vec
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, interval: Interval, hit_data: &mut HitData) -> bool {
        let center = self.sphere_center(ray.time);
        let origin_gap = center - ray.origin;
        // Quadratic constants for solving the ray intersection equation
        let a = ray.direction.length_squared();
        // Use scaled parameter for symplicity h = b / -2
        let h = Vec3::dot(ray.direction, origin_gap);
        let c = origin_gap.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return false;
        }

        let mut ray_hit = (h - discriminant.sqrt()) / a;
        if !interval.surrounds(ray_hit) {
            ray_hit = (h + discriminant.sqrt()) / a;
            if !interval.surrounds(ray_hit) {
                return false;
            }
        }

        hit_data.hit_along_ray = ray_hit;
        hit_data.point = ray.at(ray_hit);
        let outward_normal = (1. / self.radius) * (hit_data.point - center);
        hit_data.set_face_normal(ray, outward_normal);
        hit_data.material = Some(self.material);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
