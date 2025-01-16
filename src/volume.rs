use rand::random;

use crate::{
    hittable::Hittable,
    interval::Interval,
    material::{Isotropic, Material},
    texture::Texture,
    utilities::random_num,
    vec3::Vec3,
};

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inverse_density: f32,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, density: f32, texture: Texture) -> Self {
        Self {
            boundary,
            neg_inverse_density: -1. / density,
            phase_function: Material::Isotropic(Isotropic::new(texture)),
        }
    }

    pub fn new_from_color(boundary: Box<dyn Hittable>, density: f32, color: Vec3) -> Self {
        Self {
            boundary,
            neg_inverse_density: -1. / density,
            phase_function: Material::Isotropic(Isotropic::new_from_color(color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        ray: crate::ray::Ray,
        interval: crate::interval::Interval,
        hit_data: &mut crate::hittable::HitData,
    ) -> bool {
        let mut hit_data_clone1 = hit_data.clone();
        let mut hit_data_clone2 = hit_data.clone();

        if !(self
            .boundary
            .hit(ray, Interval::UNIVERSE, &mut hit_data_clone1))
        {
            return false;
        }

        // Ensure that it is not a glancing, tangential hit and that the ray will enter the volume through the
        // boundary and then exit later with another hit after
        if !(self.boundary.hit(
            ray,
            Interval::new(hit_data_clone1.hit_along_ray + 0.001, f32::INFINITY),
            &mut hit_data_clone2,
        )) {
            return false;
        }

        hit_data_clone1.hit_along_ray = f32::max(hit_data_clone1.hit_along_ray, interval.min);
        hit_data_clone2.hit_along_ray = f32::min(hit_data_clone2.hit_along_ray, interval.max);

        if hit_data_clone1.hit_along_ray >= hit_data_clone2.hit_along_ray {
            return false;
        }

        hit_data_clone1.hit_along_ray = f32::max(hit_data_clone1.hit_along_ray, 0.);

        let distance_inside_boundary = (hit_data_clone2.hit_along_ray
            - hit_data_clone1.hit_along_ray)
            * ray.direction.length();
        let hit_distance = self.neg_inverse_density * random_num().ln();

        // No scattering occurs if the randomly generated number qualifies and the ray passes straight through the volume
        // as if it doesn't hit
        if hit_distance > distance_inside_boundary {
            return false;
        }

        hit_data.hit_along_ray =
            hit_data_clone1.hit_along_ray + hit_distance / ray.direction.length();
        hit_data.point = ray.at(hit_data.hit_along_ray);

        hit_data.normal = Vec3::new(1., 0., 0.);
        hit_data.front_face = true;
        hit_data.material = Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.boundary.bounding_box()
    }
}
