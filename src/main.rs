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

    let material_ground = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.),
    });
    let material_center = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    let material_left = Material::Dielectric(Dielectric {
        refractive_index: 1.5,
    });
    let material_bubble = Material::Dielectric(Dielectric {
        refractive_index: 1. / 1.5,
    });
    let material_right = Material::Metal(Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.,
    });

    world.add(Box::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0., 0., -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.4,
        material_bubble,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        material_right,
    )));

    // Camera
    let camera = Camera::new(
        16. / 9.,
        400,
        100,
        50,
        20.,
        Vec3::new(-2., 2., 1.),
        Vec3::new(0., 0., -1.),
        Vec3::new(0., 1., 0.),
    );
    camera.render(&world);
}
