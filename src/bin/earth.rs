use raytracer::camera::Camera;
use raytracer::hittable::{HittableList, Sphere};
use raytracer::material::{Lambertian, Material};
use raytracer::texture::{ImageTexture, Texture};
use raytracer::vec3::Vec3;

fn main() {
    let mut world = HittableList::default();

    let Ok(earth_texture) = ImageTexture::new("earth.jpg".to_string()) else {
        eprintln!("Couldn't load texture");
        return
    };
    let earth_material = Material::Lambertian(Lambertian::new(Texture::Image(earth_texture)));

    let center = Vec3::new(0.,0.,0.);
    world.add(Box::new(Sphere::new(center, center, 2., earth_material)));


    let camera = Camera::new(
        16. / 9.,
        400,
        100,
        50,
        20.,
        Vec3::new(0., 0., 12.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        0.,
        10.,
        None,
    );

    camera.render(&world)
}