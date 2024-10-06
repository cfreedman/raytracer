use raytracer::{bvh::Bvh, camera::Camera, hittable::{HittableList, Sphere}, material::{Lambertian, Material}, texture::{CheckerTexture, Texture}, vec3::Vec3};


fn main() {
    let mut world = HittableList::default();

    let checker_texture =
        CheckerTexture::new_from_colors(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let material_ground_1 = Material::Lambertian(Lambertian::new(Texture::Checker(checker_texture)));
    let material_ground_2 = material_ground_1.clone();
    
    let center1 = Vec3::new(0., -10., 0.);
    let center2 = Vec3::new(0., 10., 0.);

    world.add(Box::new(Sphere::new(center1, center1, 10., material_ground_1)));
    world.add(Box::new(Sphere::new(center2, center2, 10., material_ground_2)));

    let root_node = Bvh::new(world.objects);
    let bbox = root_node.bbox;
    world = HittableList {
        objects: vec![Box::new(root_node)],
        bbox,
    };
    
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
    );

    camera.render(&world)
}