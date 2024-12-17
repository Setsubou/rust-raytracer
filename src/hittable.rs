use crate::{hit_record::HitRecord, ray::Ray, shapes::interval::Interval};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
}
