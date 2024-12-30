use itertools::Itertools;
use rand::prelude::*;

use crate::{utilities::random_num, vec3::Vec3};

#[derive(Clone)]
pub struct Perlin {
    permutation_x: [usize; 256],
    permutation_y: [usize; 256],
    permutation_z: [usize; 256],
    random_array: [f32; 256],
    random_vectors: [Vec3; 256],
}

impl Perlin {
    fn permute(mut point: [usize; 256], swaps: usize) -> [usize; 256] {
        for i in (0..swaps).rev() {
            let draw = rand::thread_rng().gen_range(0..(i + 1));
            point.swap(i,draw);

        }

        point
    }

    fn generate_permutation() -> [usize; 256] {
        let point: [usize; 256] = core::array::from_fn(|i| i);

        Self::permute(point, 256)
    }

    pub fn new() -> Perlin {
        let random_array: [f32; 256] = core::array::from_fn(|_| random_num());
        let random_vectors = core::array::from_fn(|_| Vec3::random());


        let permutation_x = Self::generate_permutation();
        let permutation_y = Self::generate_permutation();
        let permutation_z = Self::generate_permutation();

        Self {
            permutation_x,
            permutation_y,
            permutation_z,
            random_array,
            random_vectors,
        }
    }

    pub fn noise(&self, point: Vec3) -> f32 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as isize;
        let j = point.y.floor() as isize;
        let k = point.z.floor() as isize;

        let mut c = [[[Vec3::ZERO; 2]; 2]; 2];

        for indices in (0..3).map(|_i| 0..2).multi_cartesian_product() {
            let (di, dj, dk) = (indices[0], indices[1], indices[2]);
            c[di][dj][dk] = self.random_vectors[self.permutation_x[((i + di as isize) & 255) as usize]^self.permutation_y[((j + dj as isize) & 255) as usize]^self.permutation_z[((k + dk as isize) & 255) as usize]]
        }

        let res = Self::perlin_interpolate(c, u, v, w);
        res
    }

    fn perlin_interpolate(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u*u*(3.-2.*u);
        let vv = v*v*(3.-2.*v);
        let ww = w*w*(3.-2.*w);
        let mut total = 0.;

        for indices in (0..3).map(|_i| 0..2).multi_cartesian_product() {
            let (i, j, k) = (indices[0] as f32, indices[1] as f32, indices[2] as f32);

            total += (i*uu + (1.-i)*(1.-uu))*(j*vv + (1.-j)*(1.-vv))*(k*ww + (1.-k)*(1.-ww))*(Vec3::dot(c[indices[0]][indices[1]][indices[2]], Vec3::new(u-i,v-j,w-k)));
        }
        total
    }

    pub fn turbulence(&self, point: Vec3, depth: usize) -> f32 {
        let res = (0..depth).map(|i| (0.5_f32.powf(i as f32), 2_f32.powf(i as f32)*point)).fold(0., |total,(weight,point)| -> f32 {total + weight*self.noise(point)});
        res.abs()
    }
}