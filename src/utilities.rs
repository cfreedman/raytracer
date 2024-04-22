use rand::prelude::random;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.
}

pub fn random_in_interval(min: f32, max: f32) -> f32 {
    return min + (max - min) * random::<f32>();
}
