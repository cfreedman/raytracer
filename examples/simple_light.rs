use raytracer::{camera::Camera, hittable::{HittableList, Sphere}, material::{DiffuseLight, Lambertian, Material}, quad::Quad, texture::{PerlinTexture, SolidTexture, Texture}, vec3::Vec3};

fn main() {
    let mut world = HittableList::default();

    let perlin_texture = PerlinTexture::new(4.);
    let perlin_material = Material::Lambertian(Lambertian::new(Texture::Perlin(perlin_texture)));
    let ground_material = perlin_material.clone();

    let ground_center = Vec3::new(0., -1000., 0.);
    world.add(Box::new(Sphere::new(ground_center, ground_center, 1000., ground_material)));
    world.add(Box::new(Sphere::new(Vec3::new(0.,2.,0.), Vec3::new(0.,2.,0.), 2., perlin_material)));

    let diffuse_light = Material::DiffuseLight(DiffuseLight::new(Texture::Solid(SolidTexture::new(Vec3::new(4.,4.,4.)))));
    world.add(Box::new(Quad::new(Vec3::new(3.,1.,-2.), Vec3::new(2.,0.,0.), Vec3::new(0.,2.,0.), diffuse_light)));

    let camera = Camera::new(
        16. / 9.,
        400,
        100,
        50,
        20.,
        Vec3::new(26., 3., 6.),
        Vec3::new(0., 2., 0.),
        Vec3::new(0., 1., 0.),
        0.,
        10.,
        Some(Vec3::ZERO),
    );

    camera.render(&world)
}