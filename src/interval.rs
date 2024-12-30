#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f32::INFINITY,
            max: -f32::INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn new_from_intervals(a: Self, b: Self) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max <= b.max { b.max } else { a.max };

        Self::new(min, max)
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        };
        if x > self.max {
            return self.max;
        };
        x
    }

    pub fn expands(&self, delta: f32) -> Self {
        Self::new(self.min - delta / 2., self.max + delta / 2.)
    }

    pub const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: -f32::INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        min: -f32::INFINITY,
        max: f32::INFINITY,
    };

    pub const UNIT: Self = Self {
        min: 0.,
        max: 1.,
    };
}
