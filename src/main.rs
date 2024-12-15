pub mod color;
pub mod hit_record;
pub mod hittable;
pub mod hittable_list;
pub mod point;
pub mod ray;
pub mod camera;
pub mod shapes;
pub mod util;
pub mod vec3;

use camera::camera::Camera;
use color::write_color;
use indicatif::ProgressBar;
use log::info;
use point::Point3;
use ray::Ray;
use shapes::sphere::Sphere;
use simple_logger::SimpleLogger;
use std::{fs::File, io::Write, rc::Rc};
use vec3::Vec3;

// TODO: Maybe create a new config struct to store all the settings



fn main() {
    SimpleLogger::new().init().unwrap();

    // World
    let mut world = hittable_list::HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point3 {
            element: [0.0, 0.0, -1.0],
        },
        0.5,
    )));

    world.add(Rc::new(Sphere::new(
        Point3 {
            element: [0.0, -100.5, -1.0],
        },
        100.0,
    )));

    let mut camera = Camera::new(720, 16.0 / 9.0);

    camera.render(&mut world);
}
