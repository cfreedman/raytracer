use crate::{
    aabb::Aabb, hittable::{HitData, Hittable}, interval::Interval, material::Material, ray::Ray, vec3::Vec3
};

pub struct Quad {
    corner: Vec3,
    first_vector: Vec3,
    second_vector: Vec3,
    normal: Vec3,
    unscaled_normal: Vec3,
    plane_constant: f32,
    planar_coordinate_term: Vec3,
    material: Material,
    bbox: Aabb,
}

impl Quad {
    pub fn new(corner: Vec3, first_vector: Vec3, second_vector: Vec3, material: Material) -> Quad {
        // Calculation of bounding box from quad diagonal segments
        let diagonal_1_box = Aabb::new_from_points(corner, corner + first_vector + second_vector);
        let diagonal_2_box = Aabb::new_from_points(corner + first_vector, corner + second_vector);

        // Calculation of plane in 3D space containing the quad
        let unscaled_normal = Vec3::cross(first_vector, second_vector);
        let normal = unscaled_normal.unit();
        let plane_constant = Vec3::dot(unscaled_normal, corner);

        // Cached value for calculating the planar coordinates of a ray intersection in the quad plane
        let planar_coordinate_term = (1./Vec3::dot(unscaled_normal, unscaled_normal))*unscaled_normal;

        Self {
            corner,
            first_vector,
            second_vector,
            unscaled_normal,
            normal,
            plane_constant,
            planar_coordinate_term,
            material,
            bbox: Aabb::new_from_boxes(diagonal_1_box, diagonal_2_box),
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: Ray, interval: Interval, hit_data: &mut HitData) -> bool {
        // Calculation to see where ray intersects the plane containing the quad
        let denominator = Vec3::dot(self.unscaled_normal, ray.direction);

        // Rule out ray parallel to plane -> no intersection
        if denominator.abs() < 1e-8 {
            return false;
        }

        let t_intersection =
            (self.plane_constant - Vec3::dot(self.unscaled_normal, ray.origin)) / denominator;
        // Rule out intersection occurring outside of the desingated interval
        if !interval.contains(t_intersection) {
            return false;
        }

        let point_intersection = ray.at(t_intersection);
        let alpha = Vec3::dot(self.planar_coordinate_term, Vec3::cross(point_intersection - self.corner, self.second_vector));
        let beta = Vec3::dot(self.planar_coordinate_term, Vec3::cross(self.first_vector, point_intersection - self.corner));

        if !Interval::UNIT.contains(alpha) || !Interval::UNIT.contains(beta) {
            return false;
        }

        hit_data.hit_along_ray = t_intersection;
        hit_data.point = point_intersection;
        hit_data.material = Some(self.material.clone());
        hit_data.set_face_normal(ray, self.normal);
        hit_data.u = alpha;
        hit_data.v = beta;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
