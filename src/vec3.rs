// Defining Vec3 class

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
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

fn add(first_vec: &Vec3, second_vec: &Vec3) -> Vec3 {
    Vec3 {
        x: first_vec.x + second_vec.x,
        y: first_vec.y + second_vec.y,
        z: first_vec.z + second_vec.z,
    }
}

fn minus(first_vec: &Vec3, second_vec: &Vec3) -> Vec3 {
    Vec3 {
        x: first_vec.x - second_vec.x,
        y: first_vec.y - second_vec.y,
        z: first_vec.z - second_vec.z,
    }
}

fn vec_mult(first_vec: &Vec3, second_vec: &Vec3) -> Vec3 {
    Vec3 {
        x: first_vec.x * second_vec.x,
        y: first_vec.y * second_vec.y,
        z: first_vec.z * second_vec.z,
    }
}

fn constant_mult(vec: &Vec3, constant: &f32) -> Vec3 {
    Vec3 {
        x: vec.x * constant,
        y: vec.y * constant,
        z: vec.z * constant,
    }
}

fn dot_product(first_vec: &Vec3, second_vec: &Vec3) -> f32 {
    first_vec.x * second_vec.x + first_vec.y * second_vec.y + first_vec.z * second_vec.z
}

fn cross_product(first_vec: &Vec3, second_vec: &Vec3) -> Vec3 {
    Vec3 {
        x: first_vec.y * second_vec.z - first_vec.z * second_vec.y,
        y: first_vec.z * second_vec.x - first_vec.x * second_vec.z,
        z: first_vec.x * second_vec.y - first_vec.y * second_vec.z,
    }
}

fn unit_vec(vec: &Vec3) -> Vec3 {
    constant_mult(vec, &((1 as f32) / vec.length()))
}
