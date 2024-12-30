use raytracer::{camera::Camera, hittable::HittableList, material::{Lambertian, Material}, quad::Quad, vec3::Vec3};

fn main() {
    let mut world = HittableList::default();

    // Materials
    let left_red = Material::Lambertian(Lambertian::new_from_color(Vec3::new(1.,0.2,0.2)));
    let back_green = Material::Lambertian(Lambertian::new_from_color(Vec3::new(0.2,1.,0.2)));
    let right_blue = Material::Lambertian(Lambertian::new_from_color(Vec3::new(0.2,0.2,1.)));
    let upper_orange = Material::Lambertian(Lambertian::new_from_color(Vec3::new(1.,0.5,0.)));
    let lower_teal = Material::Lambertian(Lambertian::new_from_color(Vec3::new(0.2,0.8,0.8)));

    // Quads
    world.add(Box::new(Quad::new(Vec3::new(-3.,-2.,5.),Vec3::new(0.,0.,-4.),Vec3::new(0.,4.,0.),left_red)));
    world.add(Box::new(Quad::new(Vec3::new(-2.,-2.,0.),Vec3::new(4.,0.,0.),Vec3::new(0.,4.,0.),back_green)));
    world.add(Box::new(Quad::new(Vec3::new(3.,-2.,1.),Vec3::new(0.,0.,4.),Vec3::new(0.,4.,0.),right_blue)));
    world.add(Box::new(Quad::new(Vec3::new(-2.,3.,1.),Vec3::new(4.,0.,0.),Vec3::new(0.,0.,4.),upper_orange)));
    world.add(Box::new(Quad::new(Vec3::new(-2.,-3.,5.),Vec3::new(4.,0.,0.),Vec3::new(0.,0.,-4.),lower_teal)));

    // Camera
    let camera = Camera::new(
        1.,
        400,
        100,
        50,
        80.,
        Vec3::new(0.,0.,9.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        0.,
        10.,
        None
    );
    camera.render(&world);
}