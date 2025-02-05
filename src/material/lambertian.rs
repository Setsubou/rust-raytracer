use crate::{color::Color, ray::Ray, vec3::generate_random_unit_vector};

use super::material::Material;

pub struct Lambertian {
    albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &crate::ray::Ray, hit_record: &crate::hit_record::HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + generate_random_unit_vector();
        
        // Catch rogue scatter direction that is close to zero
        if scatter_direction.close_to_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered_ray = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian{albedo}
    }
}