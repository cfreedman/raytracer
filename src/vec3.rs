use std::ops::{Add, Mul, Sub};

// Defining Vec3 class
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    fn minus(&mut self) {
        self.x *= -1 as f32;
        self.y *= -1 as f32;
        self.z *= -1 as f32;
    }

    fn add(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    fn constant_mult(&mut self, constant: &f32) {
        self.x *= constant;
        self.y *= constant;
        self.z *= constant;
    }

    fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
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

pub fn dot(first_vec: Vec3, second_vec: Vec3) -> f32 {
    first_vec.x * second_vec.x + first_vec.y * second_vec.y + first_vec.z * second_vec.z
}

pub fn cross(first_vec: Vec3, second_vec: Vec3) -> Vec3 {
    Vec3 {
        x: first_vec.y * second_vec.z - first_vec.z * second_vec.y,
        y: first_vec.z * second_vec.x - first_vec.x * second_vec.z,
        z: first_vec.x * second_vec.y - first_vec.y * second_vec.z,
    }
}

pub fn unit_vec(vec: Vec3) -> Vec3 {
    (1. / vec.length()) * vec
}
