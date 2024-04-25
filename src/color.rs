use crate::interval::*;
use crate::vec3::Vec3;

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }
    0.
}

pub fn write_color(color: Vec3) -> Vec<u8> {
    // Apply gamma transformation
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    // Transform [0,1] values into byte [0,255] range
    let intensity = Interval::new(0., 0.999);
    let rbyte = (256. * intensity.clamp(r)) as u32;
    let gbyte = (256. * intensity.clamp(g)) as u32;
    let bbyte = (256. * intensity.clamp(b)) as u32;

    let color_string = format!("{} {} {}\n", rbyte, gbyte, bbyte);

    return color_string.as_bytes().to_vec();
}
