use crate::{
    aabb::Aabb,
    hittable::{HitData, Hittable},
    interval::{self, Interval},
    ray::Ray,
    vec3::Vec3,
};

pub struct Quad {
    corner: Vec3,
    first_vector: Vec3,
    second_vector: Vec3,
    normal: Vec3,
    plane_constant: f32,
    bbox: Aabb,
}

impl Quad {
    pub fn new(corner: Vec3, first_vector: Vec3, second_vector: Vec3) -> Quad {
        // Calculation of bounding box from quad diagonal segments
        let diagonal_1_box = Aabb::new_from_points(corner, corner + first_vector + second_vector);
        let diagonal_2_box = Aabb::new_from_points(corner + first_vector, corner + second_vector);

        // Calculation of plane in 3D space containing the quad
        let normal = Vec3::cross(first_vector, second_vector);
        let plane_constant = Vec3::dot(normal, corner);

        Self {
            corner,
            first_vector,
            second_vector,
            normal,
            plane_constant,
            bbox: Aabb::new_from_boxes(diagonal_1_box, diagonal_2_box),
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: Ray, interval: Interval, hit_data: &mut HitData) -> bool {
        // Calculation to see where ray intersects the plane containing the quad
        let denominator = Vec3::dot(self.normal, ray.direction);

        // Rule out ray parallel to plane -> no intersection
        if denominator.abs() < 1e-8 {
            return false;
        }

        let t_intersection =
            (self.plane_constant - Vec3::dot(self.normal, ray.origin)) / denominator;
        // Rule out intersection occurring outside of the desingated interval
        if !interval.contains(t_intersection) {
            return false;
        }

        let point_intersection = ray.at(t_intersection);

        hit_data.hit_along_ray = t_intersection;
        hit_data.point = point_intersection;
        hit_data.material = None;
        hit_data.set_face_normal(ray, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
