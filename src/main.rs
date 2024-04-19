mod camera;
use camera::Camera;
mod color;
mod hittable;
use hittable::{HittableList, Sphere};
mod interval;
mod ray;
mod vec3;
use vec3::*;

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
    }));

    // Camera
    let camera = Camera::new(16. / 9., 400);
    camera.render(&world);
}
