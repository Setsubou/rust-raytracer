use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use crate::util::{random_double, random_double_with_range};

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Vec3 {
    pub element: [f64; 3],
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            element: [
                self.element[0] + rhs.element[0],
                self.element[1] + rhs.element[1],
                self.element[2] + rhs.element[2],
            ],
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            element: [
                self.element[0] - rhs.element[0],
                self.element[1] - rhs.element[1],
                self.element[2] - rhs.element[2],
            ],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            element: [
                self.element[0] * rhs.element[0],
                self.element[1] * rhs.element[1],
                self.element[2] * rhs.element[2],
            ],
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self {
            element: [
                self.element[0] + rhs.element[0],
                self.element[1] + rhs.element[1],
                self.element[2] + rhs.element[2],
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            element: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            element: [
                self.element[0] * (1.0 / rhs),
                self.element[1] * (1.0 / rhs),
                self.element[2] * (1.0 / rhs),
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            element: [
                self.element[0] * rhs,
                self.element[1] * rhs,
                self.element[2] * rhs,
            ],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { element: [x, y, z] }
    }

    pub fn zero() -> Vec3 {
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

    pub fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    // Return true if all vector dimension is close to zero
    pub fn close_to_zero(&self) -> bool {
        let s = 1e-8;

        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }

    pub fn dot_product(&self, rhs: Vec3) -> f64 {
        (self.element[0] * rhs.element[0])
            + (self.element[1] * rhs.element[1])
            + (self.element[2] * rhs.element[2])
    }
    
    pub fn cross_product(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            (self.y() * rhs.z()) - (self.z() * rhs.y()),
            (self.z() * rhs.x()) - (self.x() * rhs.z()),
            (self.x() * rhs.y()) - (self.y() * rhs.x()),
        )
    }
}

pub fn generate_random_vector() -> Vec3 {
    Vec3::new(random_double(), random_double(), random_double())
}

pub fn generate_random_vector_with_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_double_with_range(min, max),
        random_double_with_range(min, max),
        random_double_with_range(min, max),
    )
}

pub fn generate_random_unit_vector() -> Vec3 {
    loop {
        let vector = generate_random_vector_with_range(-1.0, 1.0);
        let length_squared = vector.length_squared();

        if 1e-160 < length_squared && length_squared <= 1.0 {
            return vector / length_squared.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = generate_random_unit_vector();

    if normal.dot_product(on_unit_sphere) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_double_with_range(-1.0, 1.0),
            random_double_with_range(-1.0, 1.0),
            0.0,
        );

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[cfg(test)]
mod vector_initialization {
    use super::*;

    #[test]
    fn init_zero_vector() {
        let v = Vec3::zero();

        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), 0.0);
    }

    #[test]
    fn init_vector() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }
}

#[cfg(test)]
mod vector_math {
    use super::*;

    #[test]
    fn vector_add_with_vector() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.1, 5.2, 11.0);

        assert_eq!(v1 + v2, Vec3::new(3.1, 7.2, 14.0));
    }

    #[test]
    fn vector_sub_with_vector() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.1, 5.2, 11.0);

        assert_eq!(v1 - v2, Vec3::new(-1.1, -3.2, -8.0));
    }

    #[test]
    fn vector_mult_with_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v * 10.0, Vec3::new(10.0, 20.0, 30.0));
    }

    #[test]
    fn vector_div_with_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }
}
#[cfg(test)]
mod vector_operations {

    use approx::abs_diff_eq;

    use super::*;

    #[test]
    fn get_x() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.x(), 1.0);
    }

    #[test]
    fn get_y() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.y(), 2.0);
    }

    #[test]
    fn get_z() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn negate_vector() {
        let v = -Vec3::new(5.0, -2.0, 0.0);

        assert_eq!(v.x(), -5.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 0.0);
    }

    #[test]
    fn calculate_length_squared() {
        let v = Vec3::new(1.0, 2.0, -3.0);

        assert_eq!(v.length_squared(), 14.0);
    }

    #[test]
    fn calculate_vector_length() {
        let v = Vec3::new(1.0, 2.0, -3.0);

        assert!(abs_diff_eq!(
            v.length(),
            3.7416573867739413,
            epsilon = f64::EPSILON
        ));
    }

    #[test]
    fn calculate_unit_vector() {
        let v = Vec3::new(1.0, 2.0, -3.0);

        assert_eq!(
            v.unit_vector(),
            Vec3::new(0.2672612419124244, 0.5345224838248488, -0.8017837257372732)
        );
    }

    #[test]
    fn calculate_dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v1.dot_product(v2), 20.0);
    }
}

#[cfg(test)]
mod vector_static_functions {
    use super::*;

    #[test]
    fn calculate_dot_vector() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(u.dot_product(v), 20.0);
    }

    #[test]
    fn calculate_unit_vector() {
        let v = Vec3::new(1.0, 2.0, -3.0);

        assert_eq!(
            v.unit_vector(),
            Vec3::new(0.2672612419124244, 0.5345224838248488, -0.8017837257372732)
        );
    }
}
