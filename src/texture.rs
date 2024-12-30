use std::path::Path;

use image::{ImageError, ImageReader, RgbImage};

use crate::{interval::Interval, perlin::Perlin, vec3::Vec3};

#[derive(Clone)]
pub enum Texture {
    Solid(SolidTexture),
    Checker(CheckerTexture),
    Image(ImageTexture),
    Perlin(PerlinTexture),
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
            Self::Image(image_texture) => image_texture.value(u, v, point),
            Self::Perlin(perline_texture) => perline_texture.value(u, v, point),
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

#[derive(Clone)]
pub struct ImageTexture {
    image: RgbImage,
}
    
impl ImageTexture {
    pub fn new(file: String) -> Result<ImageTexture, ImageError> {
        let image = ImageReader::open(Path::new(&file))?.decode()?.into_rgb8();
        
        Ok(Self {
            image
        })

    }

    pub fn value(&self, mut u: f32, mut v: f32, _point: Vec3) -> Vec3 {
        if self.image.height() <= 0 {
            return Vec3::new(0., 1., 1.)
        }

        u = Interval::new(0.,1.).clamp(u);
        v = 1.0 - Interval::new(0.,1.).clamp(v);

        let i = (u as u32) * (self.image.width() - 1);
        let j = (v as u32)  * (self.image.height() - 1);
        let pixel = self.image.get_pixel(i,j);

        let color_scale = 1. / 255.;
        color_scale * Vec3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32)
    }
}


#[derive(Clone)]
pub struct PerlinTexture {
    noise: Perlin,
    scale: f32,
}

impl PerlinTexture {
    pub fn new(scale: f32) -> PerlinTexture {
        Self {
            noise: Perlin::new(),
            scale
        }
    }
    
    pub fn value(&self, mut _u: f32, mut _v: f32, point: Vec3) -> Vec3 {
        (1. + f32::sin(self.scale*point.z + 10.*self.noise.turbulence(point, 7))) * Vec3::new(0.5,0.5,0.5)
    }
}