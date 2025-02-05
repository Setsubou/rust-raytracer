use log::error;

use crate::{color::Color, ray::Ray, util::random_double, vec3::{dot_product, unit_vector, Vec3}};

use super::{material::Material, metal::Metal};

pub struct Dielectric {
    ior: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &crate::ray::Ray, hit_record: &crate::hit_record::HitRecord, attenuation: &mut Color, scattered_ray: &mut crate::ray::Ray) -> bool {
        // Looks like the scattered ray is wrong, causing to render incorrectly
        // Changing the direction manually changes the result

        let refraction_ratio = if hit_record.is_front_face {1.0 / self.ior} else {self.ior};

        let unit_direction = unit_vector(ray_in.direction());
        let cos_theta = dot_product(-unit_direction, hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || schlick_reflectance(refraction_ratio, cos_theta) > random_double() {
            direction = Metal::reflect(unit_direction, hit_record.normal);
        } else {
            direction = Dielectric::refract(unit_direction, hit_record.normal, refraction_ratio)
        }

        *attenuation = Color::WHITE;
        *scattered_ray = Ray::new(hit_record.point, direction);

        true
    }
}

impl Dielectric {
    pub fn new(ior: f64) -> Dielectric {
        Dielectric {ior}
    }

    fn refract(ray: Vec3, normal: Vec3, refraction_index: f64) -> Vec3 {
        if ray.length() - 1.0 > f64::EPSILON {
            error!(
                "Invalid vector, it must be a unit vector, it's length are {}",
                ray.length()
            );
            std::process::exit(0);
        }

        let cos_theta = f64::min(dot_product(-ray, normal), 1.0);
        let r_out_perp =  refraction_index * (ray + cos_theta * normal);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * normal;

        r_out_perp + r_out_parallel
    }
}

fn schlick_reflectance(ior: f64, cosine: f64) -> f64 {
    let r = ((1.0 - ior) / (1.0 + ior)).powi(2);

    r + (1.0 - r) * (1.0 - cosine).powi(5)
}

