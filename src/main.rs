pub mod camera;
pub mod color;
pub mod hit_record;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod point;
pub mod ray;
pub mod shapes;
pub mod util;
pub mod vec3;

use camera::Camera;
use color::Color;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use point::Point3;
use shapes::sphere::Sphere;
use simple_logger::SimpleLogger;
use std::rc::Rc;
use util::{random_double, random_double_with_range};
use vec3::Vec3;

// TODO: Maybe create a new config struct to store all the settings

fn main() {
    SimpleLogger::new().init().unwrap();

    // World
    let mut world = hittable_list::HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => {
                        // Diffuse

                        let albedo = Color::random();
                        let sphere_material = Rc::new(Lambertian::new(albedo));
                        let center2 =
                            center + Vec3::new(0.0, random_double_with_range(0.0, 0.5), 0.0);
                        world.add(Rc::new(Sphere::new(center2, 0.2, sphere_material)));
                    }
                    x if x < 0.95 => {
                        // Metal

                        let albedo = Color::random();
                        let fuzz = random_double_with_range(0.0, 0.5);
                        let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                        world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    _ => {
                        // Glass

                        let sphere_material = Rc::new(Dielectric::new(1.5));
                        world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut camera = Camera::new(
        640,
        16.0 / 9.0,
        5,
        5,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    camera.render(&mut world);
}
