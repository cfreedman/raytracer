use crate::interval::*;
use crate::vec3::Vec3;

pub fn write_color(color: Vec3) -> Vec<u8> {
    let intensity = Interval::new(0., 0.999);
    let rbyte = (256. * intensity.clamp(color.x)) as u32;
    let gbyte = (256. * intensity.clamp(color.y)) as u32;
    let bbyte = (256. * intensity.clamp(color.z)) as u32;

    let color_string = format!("{} {} {}\n", rbyte, gbyte, bbyte);

    return color_string.as_bytes().to_vec();
}
