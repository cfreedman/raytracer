#[derive(Copy, Clone, Debug)]
pub struct Interval {
    min: f32,
    max: f32,
}

impl Default for Interval {
    fn default(&self) -> Self {
        Self {
            min: 10000.,  // Change to +infinity
            max: -10000., // Change to -infinity
        }
    }
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self) -> bool {
        self.min < x && x < self.max
    }

    pub const EMPTY: Self = Self::default();
    pub const UNIVERSE: Self = Self::new(-10000., 10000.); // Change to infinity
}
