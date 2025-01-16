use raytracer::{camera::Camera, hittable::{HittableList, Sphere}, material::{Lambertian, Material}, texture::{PerlinTexture, Texture}, vec3::Vec3};



fn main() {
    let mut world = HittableList::default();

    
    let perlin_texture = PerlinTexture::new(4.);
    let perlin_material = Material::Lambertian(Lambertian::new(Texture::Perlin(perlin_texture)));
    let ground_material = perlin_material.clone();

    let ground_center = Vec3::new(0., -1000., 0.);
    world.add(Box::new(Sphere::new(ground_center, ground_center, 1000., ground_material)));
    world.add(Box::new(Sphere::new(Vec3::new(0.,2.,0.), Vec3::new(0.,2.,0.), 2., perlin_material)));

    let camera = Camera::new(
        16. / 9.,
        400,
        100,
        50,
        20.,
        Vec3::new(13., 2., 3.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        0.,
        10.,
        None,
    );

    camera.render(&world)
}