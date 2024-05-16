use rand::prelude::random;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.
}

// Returns random number uniformly between [0,1)
pub fn random_num() -> f32 {
    return random::<f32>();
}

// Returns random number uniformly in [min, max)
pub fn random_in_interval(min: f32, max: f32) -> f32 {
    return min + (max - min) * random::<f32>();
}

// Return random int in interval inclusive
pub fn random_int(min: u32, max: u32) -> u32 {
    random_in_interval(0., max as f32 + 1.) as u32
}
