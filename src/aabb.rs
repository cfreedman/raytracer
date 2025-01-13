use std::ops::Add;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Dim, Vec3};

#[derive(Copy, Clone, Debug, Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn empty() -> Self {
        Self::new_from_intervals(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY)
    }

    pub fn universe() -> Self {
        Self::new_from_intervals(Interval::UNIVERSE, Interval::UNIVERSE, Interval::UNIVERSE)
    }

    pub fn new_from_intervals(x: Interval, y: Interval, z: Interval) -> Self {
        let mut res = Self { x, y, z };
        res.pad_to_minimums();
        res
    }

    pub fn new_from_points(a: Vec3, b: Vec3) -> Self {
        let x = if a.x < b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };

        let y = if a.y < b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };

        let z = if a.z < b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };

        Self::new_from_intervals(x, y, z)
    }

    pub fn new_from_boxes(a: Self, b: Self) -> Self {
        let x = Interval::new_from_intervals(a.x, b.x);
        let y = Interval::new_from_intervals(a.y, b.y);
        let z = Interval::new_from_intervals(a.z, b.z);

        Self::new_from_intervals(x, y, z)
    }

    pub fn get(&self, dim: Dim) -> Interval {
        match dim {
            Dim::X => self.x,
            Dim::Y => self.y,
            Dim::Z => self.z,
        }
    }

    pub fn hit(&self, ray: Ray, mut ray_interval: Interval) -> bool {
        let ray_origin = ray.origin;
        let ray_dir = ray.direction;

        for dim in Dim::ALL {
            let interval = self.get(dim);
            let scale = 1. / ray_dir.get(dim);

            let t0 = scale * (interval.min - ray_origin.get(dim));
            let t1 = scale * (interval.max - ray_origin.get(dim));

            if t0 < t1 {
                if t0 > ray_interval.min {
                    ray_interval.min = t0;
                }
                if t1 < ray_interval.max {
                    ray_interval.max = t1;
                }
            } else {
                if t1 > ray_interval.min {
                    ray_interval.min = t1;
                }
                if t0 < ray_interval.max {
                    ray_interval.max = t0;
                }
            }

            if ray_interval.max <= ray_interval.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> Dim {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                Dim::X
            } else {
                Dim::Z
            }
        } else {
            if self.y.size() > self.z.size() {
                Dim::Y
            } else {
                Dim::Z
            }
        }
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;

        if self.x.size() < delta {
            self.x.expands(delta);
        }
        if self.y.size() < delta {
            self.y.expands(delta);
        }
        if self.z.size() < delta {
            self.z.expands(delta);
        }
    }

    pub fn shift(&self, offset: Vec3) -> Self {
        Self::new_from_intervals(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }
}

impl Add<Vec3> for Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vec3) -> Self::Output {
        self.shift(rhs)
    }
}
