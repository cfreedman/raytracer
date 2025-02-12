use std::io;

use raytracer::{camera::Camera, hittable::HittableList, material::{Lambertian, Material}, quad::Quad, vec3::Vec3};

fn main() -> io::Result<()> {
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
    let camera = Camera::init()
        .aspect_ratio(1.)
        .image_width(1000)
        .samples_per_pixel(100)
        .max_depth(50)
        .vertical_fov(80.)
        .look_from(Vec3::new(0.,0.,9.))
        .look_to(Vec3::new(0.,0.,0.))
        .vec_up(Vec3::new(0.,1.,0.))
        .defocus_angle(0.)
        .focus_distance(10.)
        .build();

    camera.render_to_disc("quad", &world)?;

    Ok(())
}