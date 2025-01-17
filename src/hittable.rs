use core::f32;
use std::f32::consts::PI;

use crate::aabb::Aabb;
use crate::interval::*;
use crate::material::Material;
use crate::quad::Quad;
use crate::ray::*;
use crate::utilities::degrees_to_radians;
use crate::vec3::*;

#[derive(Clone)]
pub struct HitData {
    pub hit_along_ray: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Option<Material>,
    pub u: f32,
    pub v: f32,
}

impl Default for HitData {
    fn default() -> Self {
        Self {
            hit_along_ray: 0.,
            point: Vec3::default(),
            normal: Vec3::default(),
            front_face: false,
            material: None,
            u: 0.,
            v: 0.,
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
    pub objects: Vec<Box<dyn Hittable>>,
    pub bbox: Aabb,
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

    fn get_uv(point: Vec3) -> (f32, f32) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;

        let u = phi / 2. * PI;
        let v = theta / PI;

        (u, v)
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
        (hit_data.u, hit_data.v) = Self::get_uv(outward_normal);
        hit_data.material = Some(self.material.clone());

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct BoxObject {
    sides: HittableList,
}

impl BoxObject {
    pub fn new(point_a: Vec3, point_b: Vec3, mat: Material) -> BoxObject {
        let mins = Vec3::new(
            f32::min(point_a.x, point_b.x),
            f32::min(point_a.y, point_b.y),
            f32::min(point_a.z, point_b.z),
        );
        let maxes = Vec3::new(
            f32::max(point_a.x, point_b.x),
            f32::max(point_a.y, point_b.y),
            f32::max(point_a.z, point_b.z),
        );

        let dx = Vec3::new(maxes.x - mins.x, 0., 0.);
        let dy = Vec3::new(0., maxes.y - mins.y, 0.);
        let dz = Vec3::new(0., 0., maxes.z - mins.z);

        let mut sides = HittableList::default();
        sides.add(Box::new(Quad::new(
            Vec3::new(mins.x, mins.y, maxes.z),
            dx,
            dy,
            mat.clone(),
        )));
        sides.add(Box::new(Quad::new(
            Vec3::new(maxes.x, mins.y, maxes.z),
            -dz,
            dy,
            mat.clone(),
        )));
        sides.add(Box::new(Quad::new(
            Vec3::new(maxes.x, mins.y, mins.z),
            -dx,
            dy,
            mat.clone(),
        )));
        sides.add(Box::new(Quad::new(
            Vec3::new(mins.x, mins.y, mins.z),
            dz,
            dy,
            mat.clone(),
        )));
        sides.add(Box::new(Quad::new(
            Vec3::new(mins.x, maxes.y, maxes.z),
            dx,
            -dz,
            mat.clone(),
        )));
        sides.add(Box::new(Quad::new(
            Vec3::new(mins.x, mins.y, mins.z),
            dx,
            dz,
            mat.clone(),
        )));

        Self { sides }
    }
}

impl Hittable for BoxObject {
    fn hit(&self, ray: Ray, interval: Interval, hit_data: &mut HitData) -> bool {
        self.sides.hit(ray, interval, hit_data)
    }

    fn bounding_box(&self) -> Aabb {
        self.sides.bbox
    }
}

pub struct TranslateInstance {
    object: Box<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl TranslateInstance {
    pub fn new(object: Box<dyn Hittable>, offset: Vec3) -> TranslateInstance {
        let bbox = object.bounding_box() + offset;

        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for TranslateInstance {
    fn hit(&self, ray: Ray, interval: Interval, hit_data: &mut HitData) -> bool {
        let offset_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        if !self.object.hit(offset_ray, interval, hit_data) {
            return false;
        };

        hit_data.point += self.offset;
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct YRotationInstance {
    object: Box<dyn Hittable>,
    theta: f32,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Aabb,
}

impl YRotationInstance {
    pub fn new(object: Box<dyn Hittable>, theta: f32) -> YRotationInstance {
        let radians = degrees_to_radians(theta);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut bbox = object.bounding_box();
        let mut box_corner_1 = Vec3::ZERO;
        let mut box_corner_2 = Vec3::ZERO;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let (i, j, k) = (i as f32, j as f32, k as f32);
                    let x = i * bbox.x.max + (1. - i) * bbox.x.min;
                    let y = j * bbox.y.max + (1. - j) * bbox.y.min;
                    let z = k * bbox.z.max + (1. - k) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;
                    let temp = Vec3::new(new_x, y, new_z);

                    for c in Dim::ALL {
                        box_corner_1.set(c, f32::min(f32::INFINITY, temp.get(c)));
                        box_corner_2.set(c, f32::max(f32::NEG_INFINITY, temp.get(c)));
                    }
                }
            }
        }

        bbox = Aabb::new_from_points(box_corner_1, box_corner_2);

        Self {
            object,
            theta,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for YRotationInstance {
    fn hit(&self, ray: Ray, interval: Interval, hit_data: &mut HitData) -> bool {
        // COnstruct transformed ray
        let origin = Vec3::new(
            self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z,
            ray.origin.y,
            self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z,
        );
        let direction = Vec3::new(
            self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z,
            ray.direction.y,
            self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z,
        );

        let rotated_ray = Ray::new(origin, direction, ray.time);

        // Check intersection of transformed ray with the object
        if !self.object.hit(rotated_ray, interval, hit_data) {
            return false;
        };

        // Rotate the hit data point and normal back appropriately
        hit_data.point = Vec3::new(
            self.cos_theta * hit_data.point.x + self.sin_theta * hit_data.point.z,
            hit_data.point.y,
            -self.sin_theta * hit_data.point.x + self.cos_theta * hit_data.point.z,
        );
        hit_data.normal = Vec3::new(
            self.cos_theta * hit_data.normal.x + self.sin_theta * hit_data.normal.z,
            hit_data.normal.y,
            -self.sin_theta * hit_data.normal.x + self.cos_theta * hit_data.normal.z,
        );

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
