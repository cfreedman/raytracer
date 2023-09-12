// Defining vec3 class

pub struct vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl vec3 {
    fn minus(&mut self) {
        self.x *= -1 as f64;
        self.y *= -1 as f64;
        self.z *= -1 as f64;
    }

    fn add(&mut self, other: &vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    fn constant_mult(&mut self, constant: &f64) {
        self.x *= constant;
        self.y *= constant;
        self.z *= constant;
    }

    fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

fn add(first_vec: &vec3, second_vec: &vec3) -> vec3 {
    vec3 {
        x: first_vec.x + second_vec.x,
        y: first_vec.y + second_vec.y,
        z: first_vec.z + second_vec.z,
    }
}

fn minus(first_vec: &vec3, second_vec: &vec3) -> vec3 {
    vec3 {
        x: first_vec.x - second_vec.x,
        y: first_vec.y - second_vec.y,
        z: first_vec.z - second_vec.z,
    }
}

fn vec_mult(first_vec: &vec3, second_vec: &vec3) -> vec3 {
    vec3 {
        x: first_vec.x * second_vec.x,
        y: first_vec.y * second_vec.y,
        z: first_vec.z * second_vec.z,
    }
}

fn constant_mult(vec: &vec3, constant: &f64) -> vec3 {
    vec3 {
        x: vec.x * constant,
        y: vec.y * constant,
        z: vec.z * constant,
    }
}

fn dot_product(first_vec: &vec3, second_vec: &vec3) -> f64 {
    first_vec.x * second_vec.x + first_vec.y * second_vec.y + first_vec.z * second_vec.z
}

fn cross_product(first_vec: &vec3, second_vec: &vec3) -> vec3 {
    vec3 {
        x: first_vec.y * second_vec.z - first_vec.z * second_vec.y,
        y: first_vec.z * second_vec.x - first_vec.x * second_vec.z,
        z: first_vec.x * second_vec.y - first_vec.y * second_vec.z,
    }
}

fn unit_vec(vec: &vec3) -> vec3 {
    constant_mult(vec, &((1 as f64) / vec.length()))
}
