use std::rc::Rc;

use log::error;

use crate::{
    color::Color,
    material::{lambertian::Lambertian, material::Material},
    point::Point3,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub material: Rc<dyn Material>,
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub is_front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            is_front_face: true,
            material: Rc::new(Lambertian::new(Color::WHITE)),
        }
    }
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            is_front_face: false,
            material: Rc::new(Lambertian::new(Color::WHITE)),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        if outward_normal.length() - 1.0 > f64::EPSILON {
            error!(
                "Invalid vector, it must be a unit vector, it's length are {}",
                outward_normal.length()
            );
            std::process::exit(0);
        }

        self.is_front_face = ray.direction().dot_product(outward_normal) < 0.0;

        self.normal = if self.is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
