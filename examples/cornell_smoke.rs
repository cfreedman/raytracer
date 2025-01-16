use std::io;

use raytracer::{
    camera::Camera,
    hittable::HittableList,
    material::{DiffuseLight, Lambertian, Material},
    quad::Quad,
    texture::{SolidTexture, Texture},
    vec3::Vec3,
};

fn main() -> io::Result<()> {
    let mut world = HittableList::default();

    let red = Material::Lambertian(Lambertian::new(Texture::Solid(SolidTexture::new(
        Vec3::new(0.65, 0.05, 0.05),
    ))));
    let white = Material::Lambertian(Lambertian::new(Texture::Solid(SolidTexture::new(
        Vec3::new(0.73, 0.73, 0.73),
    ))));
    let green = Material::Lambertian(Lambertian::new(Texture::Solid(SolidTexture::new(
        Vec3::new(0.12, 0.45, 0.15),
    ))));
    let light = Material::DiffuseLight(DiffuseLight::new(Texture::Solid(SolidTexture::new(
        Vec3::new(15., 15., 15.),
    ))));

    world.add(Box::new(Quad::new(
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        green,
    )));
    world.add(Box::new(Quad::new(
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        red,
    )));
    world.add(Box::new(Quad::new(
        Vec3::new(343., 554., 332.),
        Vec3::new(-130., 0., 0.),
        Vec3::new(0., 0., -105.),
        light,
    )));
    world.add(Box::new(Quad::new(
        Vec3::new(0., 0., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Vec3::new(555., 555., 555.),
        Vec3::new(-555., 0., 0.),
        Vec3::new(0., 0., -555.),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Vec3::new(0., 0., 555.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        white.clone(),
    )));

    let camera = Camera::init()
        .aspect_ratio(1.)
        .image_width(600)
        .samples_per_pixel(100)
        .max_depth(20)
        .background(Vec3::ZERO)
        .vertical_fov(40.)
        .look_from(Vec3::new(278., 278., -800.))
        .look_to(Vec3::new(278., 278., 0.))
        .build();

    camera.render_to_disc("cornell_smoke", &world)?;
    Ok(())
}
