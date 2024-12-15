use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self.negate()
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

impl Div<f64> for &Vec3 {
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

    pub fn negate(&self) -> Vec3 {
        Vec3 {
            element: [-self.x(), -self.y(), -self.z()],
        }
    }

    pub fn length_squared(&self) -> f64 {
        f64::powf(self.x(), 2.0) + f64::powf(self.y(), 2.0) + f64::powf(self.z(), 2.0)
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn unit_vector(&self) -> Vec3 {
        unit_vector(self)
    }

    pub fn dot_product(&self, rhs: &Vec3) -> f64 {
        dot_product(self, rhs)
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot_product(u: &Vec3, v: &Vec3) -> f64 {
    u.element[0] * v.element[0] + u.element[1] * v.element[1] + u.element[2] * v.element[2]
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
        let v = Vec3::new(5.0, -2.0, 0.0).negate();

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
    fn calculate_dot_vector() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(dot_product(&u, &v), 20.0);
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
}
