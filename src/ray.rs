use crate::vec3::*;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        return add(&self.origin, &constant_mult(&self.direction, &t));
    }
}
