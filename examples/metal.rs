use std::io;

use raytracer::{
    camera::Camera,
    hittable::{HittableList, Sphere},
    material::{Lambertian, Material, Metal},
    vec3::Vec3,
};

fn main() -> io::Result<()> {
    let mut world = HittableList::default();

    let material_ground = Material::Lambertian(Lambertian::new_from_color(Vec3::new(0.8, 0.8, 0.)));
    let material_center =
        Material::Lambertian(Lambertian::new_from_color(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Material::Metal(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.));

    world.add(Box::new(Sphere::new(
        Vec3::new(0., -100.5, 0.),
        Vec3::new(0., -100.5, 0.),
        100.,
        material_ground,
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0., 0., -1.2),
        Vec3::new(0., 0., -1.2),
        0.5,
        material_center,
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        Vec3::new(-1., 0., -1.),
        0.5,
        material_left,
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(1., 0., -1.),
        Vec3::new(1., 0., -1.),
        0.5,
        material_right,
    )));

    let camera = Camera::init().build();

    camera.render_to_disc("metal", &world)?;

    Ok(())
}
