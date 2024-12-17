pub mod camera;
pub mod color;
pub mod hit_record;
pub mod hittable;
pub mod hittable_list;
pub mod point;
pub mod ray;
pub mod shapes;
pub mod util;
pub mod vec3;

use camera::Camera;
use point::Point3;
use shapes::sphere::Sphere;
use simple_logger::SimpleLogger;
use std::rc::Rc;

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
