mod camera;
use camera::Camera;
mod color;
mod hittable;
use hittable::{HittableList, Sphere};
mod interval;
mod material;
mod ray;
mod utilities;
use utilities::{random_in_interval, random_num};
mod vec3;
use material::{Dielectric, Lambertian, Material, Metal};
use vec3::*;

fn main() {
    // World
    let mut world = HittableList::default();

    let material_ground = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    });
    let ground_center = Vec3::new(0., -1000., 0.);
    world.add(Box::new(Sphere::new(
        ground_center,
        ground_center,
        1000.,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let sample_material = random_num();
            let center = Vec3::new(
                a as f32 + 0.9 * random_num(),
                0.2,
                b as f32 + 0.9 * random_num(),
            );

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if sample_material < 0.8 {
                    // Diffuse sphere spawns
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Material::Lambertian(Lambertian { albedo });
                    let center_end = center + Vec3::new(0., random_in_interval(0., 0.5), 0.);
                    world.add(Box::new(Sphere::new(
                        center,
                        center_end,
                        0.2,
                        sphere_material,
                    )));
                } else if sample_material < 0.95 {
                    // Metal sphere spawns
                    let albedo = Vec3::random_in_interval(0.5, 1.);
                    let fuzz = random_in_interval(0., 0.5);
                    let sphere_material = Material::Metal(Metal { albedo, fuzz });
                    world.add(Box::new(Sphere::new(center, center, 0.2, sphere_material)));
                } else {
                    // Glass sphere spawns
                    let sphere_material = Material::Dielectric(Dielectric {
                        refractive_index: 1.5,
                    });
                    world.add(Box::new(Sphere::new(center, center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_0 = Material::Dielectric(Dielectric {
        refractive_index: 1.5,
    });
    let center_0 = Vec3::new(0., 1., 0.);
    world.add(Box::new(Sphere::new(center_0, center_0, 1., material_0)));

    let material_1 = Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    });
    let center_1 = Vec3::new(-4., 1., 0.);
    world.add(Box::new(Sphere::new(center_1, center_1, 1., material_1)));

    let material_2 = Material::Metal(Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.,
    });
    let center_2 = Vec3::new(4., 1., 0.);
    world.add(Box::new(Sphere::new(center_2, center_2, 1., material_2)));

    // Camera
    let camera = Camera::new(
        16. / 9.,
        400,
        100,
        50,
        20.,
        Vec3::new(13., 2., 3.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        0.6,
        10.,
    );
    camera.render(&world);
}
