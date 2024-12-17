use std::rc::Rc;

use crate::{hit_record::HitRecord, hittable::Hittable, ray::Ray, shapes::interval::Interval};

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();
        let mut temp_record: HitRecord = HitRecord::new();

        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_t.min(), closest_so_far), &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record.clone();
            }
        }

        hit_anything
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}
