mod vec3;

pub fn write_color(color: &vec3::Vec3) -> &[u8] {
    format!(
        "{} {} {}\n",
        255.999 * color.x,
        255.999 * color.y,
        255.999 * color.z
    )
    .as_bytes()
}
