use crate::{hittable::HitData, ray::Ray, utilities::random_num, vec3::Vec3};

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
                Vec3::dot(scattered.direction, hit_data.normal) > 0.
            }
            Self::Dielectric(dielectric) => {
                let adjusted_ref_ratio = if hit_data.front_face {
                    1. / dielectric.refractive_index
                } else {
                    dielectric.refractive_index
                };

                let norm_incoming_vec = ray_in.direction.unit();
                let cos_theta = Vec3::dot(-1. * norm_incoming_vec, hit_data.normal).min(1.);
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();

                let direction = if adjusted_ref_ratio * sin_theta > 1.
                    || Dielectric::reflectance(cos_theta, adjusted_ref_ratio) > random_num()
                {
                    Vec3::reflect(norm_incoming_vec, hit_data.normal)
                } else {
                    Vec3::refract(ray_in.direction.unit(), hit_data.normal, adjusted_ref_ratio)
                };
                *attenuation = Vec3::new(1., 1., 1.);
                *scattered = Ray::new(hit_data.point, direction);
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

impl Dielectric {
    // Schlick approximation for reflectance
    pub fn reflectance(cosine: f32, ref_index: f32) -> f32 {
        let r0 = (1. - ref_index) / (1. + ref_index);
        r0 * r0 + (1. - r0 * r0) * (1. - cosine).powf(5.)
    }
}
