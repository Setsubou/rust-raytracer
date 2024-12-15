use log::error;

use crate::{point::Point3, ray::Ray, vec3::{dot_product, Vec3}};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
        }
    }
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // Outward normal must be a unit vector

        if outward_normal.length() - 1.0 > f64::EPSILON {
            error!("Invalid normal, it must be a unit vector, it's length are {}", outward_normal.length());
            std::process::exit(0);
        }

        self.front_face = if dot_product(&ray.direction(), &outward_normal) < f64::EPSILON {
            true
        } else {
            false
        };

        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self,ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}