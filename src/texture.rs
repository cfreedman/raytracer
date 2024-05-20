use crate::vec3::Vec3;

#[derive(Clone)]
pub enum Texture {
    Solid(SolidTexture),
    Checker(CheckerTexture),
}

impl Default for Texture {
    fn default() -> Self {
        Self::Solid(SolidTexture::default())
    }
}

impl Texture {
    pub fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        match self {
            Self::Solid(solid_texture) => solid_texture.value(u, v, point),
            Self::Checker(checker_texture) => checker_texture.value(u, v, point),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct SolidTexture {
    pub color: Vec3,
}

impl SolidTexture {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }

    pub fn value(&self, _u: f32, _v: f32, _point: Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    pub inv_scale: f32,
    pub even: Box<Texture>,
    pub odd: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f32, even: Texture, odd: Texture) -> Self {
        Self {
            inv_scale: 1. / scale,
            even: Box::new(even),
            odd: Box::new(odd),
        }
    }

    pub fn new_from_colors(scale: f32, even: Vec3, odd: Vec3) -> Self {
        Self::new(
            scale,
            Texture::Solid(SolidTexture::new(even)),
            Texture::Solid(SolidTexture::new(odd)),
        )
    }

    pub fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        let x = (self.inv_scale * point.x).floor() as i32;
        let y = (self.inv_scale * point.y).floor() as i32;
        let z = (self.inv_scale * point.z).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
