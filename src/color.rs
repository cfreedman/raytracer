use crate::vec3::Vec3;

pub fn write_color(color: &Vec3) -> Vec<u8> {
    let color_string = format!(
        "{} {} {}\n",
        255.999 * color.x,
        255.999 * color.y,
        255.999 * color.z
    );

    return color_string.as_bytes().to_vec();
}
