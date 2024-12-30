use crate::{
    hittable::HitData,
    ray::Ray,
    texture::{SolidTexture, Texture},
    utilities::random_num,
    vec3::Vec3,
};

#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
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
        hit_data: &mut HitData,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Self::Lambertian(lamb) => lamb.scatter(ray_in, hit_data, attenuation, scattered),
            Self::Metal(metal) => metal.scatter(ray_in, hit_data, attenuation, scattered),
            Self::Dielectric(dielectric) => {
                dielectric.scatter(ray_in, hit_data, attenuation, scattered)
            }
            Self::DiffuseLight(_) => false,
        }
    }

    pub fn emit(&self, point: Vec3, u: f32, v: f32) -> Vec3 {
        match self {
            Self::Lambertian(lamb) => lamb.emit(point, u, v),
            Self::Metal(metal) => metal.emit(point, u, v),
            Self::Dielectric(dielectric) => dielectric.emit(point, u, v),
            Self::DiffuseLight(light) => light.emit(point, u, v),
        }
    }
}

#[derive(Clone, Default)]
pub struct Lambertian {
    pub texture: Texture,
}

impl Lambertian {
    pub fn new(texture: Texture) -> Self {
        Self { texture }
    }

    pub fn new_from_color(color: Vec3) -> Self {
        Self::new(Texture::Solid(SolidTexture::new(color)))
    }

    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_data: &mut HitData,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_data.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter directions near zero from surface
        if scatter_direction.near_zero() {
            scatter_direction = hit_data.normal;
        };
        *scattered = Ray::new(hit_data.point, scatter_direction, ray_in.time);
        *attenuation = self.texture.value(hit_data.u, hit_data.v, hit_data.point);
        true
    }

    pub fn emit(&self, _point: Vec3, _u: f32, _v: f32) -> Vec3 {
        Vec3::ZERO
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_data: &mut HitData,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(ray_in.direction, hit_data.normal).unit()
            + self.fuzz * Vec3::random_unit_vector();
        *scattered = Ray::new(hit_data.point, reflected, ray_in.time);
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction, hit_data.normal) > 0.
    }

    pub fn emit(&self, _point: Vec3, _u: f32, _v: f32) -> Vec3 {
        Vec3::ZERO
    }
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

    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_data: &mut HitData,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let adjusted_ref_ratio = if hit_data.front_face {
            1. / self.refractive_index
        } else {
            self.refractive_index
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
        *scattered = Ray::new(hit_data.point, direction, ray_in.time);
        true
    }

    pub fn emit(&self, _point: Vec3, _u: f32, _v: f32) -> Vec3 {
        Vec3::ZERO
    }
}

#[derive(Clone, Default)]
pub struct DiffuseLight {
    texture: Texture,
}

impl DiffuseLight {
    pub fn new(texture: Texture) -> DiffuseLight {
        Self {
            texture
        }
    }

    pub fn emit(&self, point: Vec3, u: f32, v: f32) -> Vec3 {
        self.texture.value(u, v, point)
    }
}
