pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

const _EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};
const _UNIVERSE: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};
