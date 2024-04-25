use crate::{hittable::HitData, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit_data: HitData, attenuation: Vec3, scattered: Ray) -> bool {}
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: Ray,
        mut hit_data: HitData,
        mut attenuation: Vec3,
        mut scattered: Ray,
    ) -> bool {
        let mut scatter_direction = hit_data.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter directions near zero from surface
        if scatter_direction.near_zero() {
            scatter_direction = hit_data.normal;
        };
        scattered = Ray::new(hit_data.point, scatter_direction);
        attenuation = self.albedo;
        true
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: Ray,
        hit_data: HitData,
        mut attenuation: Vec3,
        mut scattered: Ray,
    ) -> bool {
        let reflected = Vec3::reflect(ray_in.direction, hit_data.normal);
        scattered = Ray::new(hit_data.point, reflected);
        attenuation = self.albedo;
        true
    }
}
