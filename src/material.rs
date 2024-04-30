use crate::{
    hittable::HitData,
    ray::Ray,
    vec3::{dot, Vec3},
};

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Lambertian::default())
    }
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_data: HitData,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Self::Lambertian(lamb) => {
                let mut scatter_direction = hit_data.normal + Vec3::random_unit_vector();

                // Catch degenerate scatter directions near zero from surface
                if scatter_direction.near_zero() {
                    scatter_direction = hit_data.normal;
                };
                *scattered = Ray::new(hit_data.point, scatter_direction);
                *attenuation = lamb.albedo;
                true
            }
            Self::Metal(metal) => {
                let reflected = Vec3::reflect(ray_in.direction, hit_data.normal).unit()
                    + metal.fuzz * Vec3::random_unit_vector();
                *scattered = Ray::new(hit_data.point, reflected);
                *attenuation = metal.albedo;
                dot(scattered.direction, hit_data.normal) > 0.
            }
            Self::Dielectric(dielectric) => {
                let adjusted_ref_ratio = if hit_data.front_face {
                    1. / dielectric.refractive_index
                } else {
                    dielectric.refractive_index
                };

                let ref_vec =
                    Vec3::refract(ray_in.direction.unit(), hit_data.normal, adjusted_ref_ratio);
                *attenuation = Vec3::new(1., 1., 1.);
                *scattered = Ray::new(hit_data.point, ref_vec);
                true
            }
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Dielectric {
    pub refractive_index: f32,
}
