pub struct Vec3 {
    element: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            element: [x, y, z],
        }
    }

    pub fn zero_vec() -> Vec3 {
        Vec3 {
            element: [0.0, 0.0, 0.0],
        }
    }

    pub fn x(&self) -> f64 {
        self.element[0]
    }

    pub fn y(&self) -> f64 {
        self.element[1]
    }

    pub fn z(&self) -> f64 {
        self.element[2]
    }

    pub fn negate(&mut self) {
        self.element = [-self.x(), -self.y(), -self.z()]
    }

    pub fn add(&mut self, other: &Vec3) {
        self.element[0] += other.x();
        self.element[1] += other.y();
        self.element[2] += other.z();
    }

    pub fn mult(&mut self, scalar: f64) {
        for element in self.element.iter_mut() {
            *element *= scalar
        }
    }

    pub fn div(&mut self, scalar: f64) {
        for element in self.element.iter_mut() {
            *element *= 1.0/scalar
        }
    }

    fn length_squared(&self) -> f64 {
        f64::powf(self.x(), 2.0) + f64::powf(self.y(), 2.0) + f64::powf(self.z(), 2.0)
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
}