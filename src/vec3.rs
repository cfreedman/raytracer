use std::ops::{Add, AddAssign, Mul, Sub};

use crate::utilities::{random_in_interval, random_num};

// Defining Vec3 class
#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn minus(&mut self) {
        self.x *= -1 as f32;
        self.y *= -1 as f32;
        self.z *= -1 as f32;
    }

    pub fn add(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn constant_mult(&mut self, constant: f32) {
        self.x *= constant;
        self.y *= constant;
        self.z *= constant;
    }

    pub fn length_squared(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Self {
        (1. / self.length()) * self
    }

    pub fn dot(first_vec: Vec3, second_vec: Vec3) -> f32 {
        first_vec.x * second_vec.x + first_vec.y * second_vec.y + first_vec.z * second_vec.z
    }

    pub fn cross(first_vec: Vec3, second_vec: Vec3) -> Vec3 {
        Vec3 {
            x: first_vec.y * second_vec.z - first_vec.z * second_vec.y,
            y: first_vec.z * second_vec.x - first_vec.x * second_vec.z,
            z: first_vec.x * second_vec.y - first_vec.y * second_vec.x,
        }
    }

    pub fn random() -> Self {
        Self::new(random_num(), random_num(), random_num())
    }

    pub fn random_in_interval(min: f32, max: f32) -> Self {
        Self::new(
            random_in_interval(min, max),
            random_in_interval(min, max),
            random_in_interval(min, max),
        )
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let sample = Self::random_in_interval(-1., 1.);
            if sample.length_squared() < 1. {
                return sample.unit();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Self) -> Self {
        let sample = Self::random_unit_vector();
        if Self::dot(sample, normal) > 0. {
            return sample;
        } else {
            return -1. * sample;
        }
    }

    // Returns true in vec is close enough in magnitude to zero
    pub fn near_zero(&self) -> bool {
        let tolerance = 10.0_f32.powf(-8.);
        self.x < tolerance && self.y < tolerance && self.z < tolerance
    }

    // Returns reflected vector from surface with normal vector bisecting angle
    // of attack
    pub fn reflect(incoming: Vec3, normal: Vec3) -> Vec3 {
        incoming - 2. * Self::dot(incoming, normal) * normal
    }

    // Returns refracted vector passing through surface
    pub fn refract(incoming: Vec3, normal: Vec3, ref_ratio: f32) -> Vec3 {
        let cos_theta = (Self::dot(-1. * incoming, normal)).min(1.);
        let ref_vec_perp = ref_ratio * (incoming + cos_theta * normal);
        let ref_vec_par = (-1. * (1. - ref_vec_perp.length_squared()).sqrt()) * normal;
        ref_vec_perp + ref_vec_par
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Self::Output::new(self * other.x, self * other.y, self * other.z)
    }
}
