use crate::{
    color::Color,
    ray::{self, Ray},
    vec3::{generate_random_unit_vector, Vec3},
};

use super::material::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &ray::Ray,
        hit_record: &crate::hit_record::HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut ray::Ray,
    ) -> bool {
        let mut reflected = Metal::reflect(ray_in.direction(), hit_record.normal);
        reflected = reflected.unit_vector() + (self.fuzz * generate_random_unit_vector());

        *scattered_ray = Ray::new(hit_record.point, reflected);
        *attenuation = self.albedo;

        scattered_ray.direction().dot_product(hit_record.normal) > 0.0
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };

        Metal { albedo, fuzz }
    }

    pub fn reflect(ray: Vec3, normal: Vec3) -> Vec3 {
        ray - 2.0 * ray.dot_product(normal) * normal
    }
}
