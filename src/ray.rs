use crate::{point::Point3, vec3::Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn at(self, scalar: f64) -> Point3 {
        self.origin + (self.direction * scalar)
    }
}