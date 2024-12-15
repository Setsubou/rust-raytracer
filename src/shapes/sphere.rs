use crate::{
    hit_record::HitRecord,
    hittable::Hittable,
    point::Point3,
    ray::Ray,
    vec3::{dot_product, unit_vector},
};

use super::interval::Interval;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin();

        let a = ray.direction().length_squared();
        let h = dot_product(&ray.direction(), &oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - (a * c);
        if discriminant < 0.0 {
            return false;
        }

        // Find the nearest root that lies in the acceptable range.
        let squared_discriminant = discriminant.sqrt();
        let root = (h - squared_discriminant) / a;

        if !(ray_t.surrounds(root)) {
            let root = (h + squared_discriminant) / a;

            if !(ray_t.surrounds(root)) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);

        let outward_normal = unit_vector(&((hit_record.point - self.center) / self.radius));
        hit_record.set_face_normal(ray, &outward_normal);

        true
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        let radius = radius.max(0.0);

        Sphere { center, radius }
    }
}
