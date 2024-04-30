mod camera;
use camera::Camera;
mod color;
mod hittable;
use hittable::{HittableList, Sphere};
mod interval;
mod material;
mod ray;
mod utilities;
mod vec3;
use material::{Dielectric, Lambertian, Material, Metal};
use vec3::*;

fn main() {
    // World
    let mut world = HittableList::default();

    // Define several materials
    let material_ground = Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.),
    };
    let material_center = Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    };
    let material_left = Dielectric {
        refractive_index: 1.5,
    };
    let material_right = Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    // Add sphere objects to world scene
    world.add(Box::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        Material::Lambertian(material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0., 0., -1.2),
        0.5,
        Material::Lambertian(material_center),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        Material::Dielectric(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        Material::Metal(material_right),
    )));

    // Camera
    let camera = Camera::new(16. / 9., 400, 100, 50);
    camera.render(&world);
}
