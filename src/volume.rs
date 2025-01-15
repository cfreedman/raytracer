use crate::{
    hittable::Hittable,
    material::{Isotropic, Material},
    texture::Texture,
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
        true
    }

    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.boundary.bounding_box()
    }
}
