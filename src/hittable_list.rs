use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};

pub(crate) struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, hit_record: &mut crate::hittable::HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_record: HitRecord = HitRecord::new();

        for object in &self.objects {
            
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
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
        HittableList { objects: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}