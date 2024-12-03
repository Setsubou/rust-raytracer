pub mod color;
pub mod point;
pub mod ray;
pub mod vec3;

use color::{write_color, Color};
use indicatif::ProgressBar;
use log::info;
use point::Point3;
use simple_logger::SimpleLogger;
use std::{fs::File, io::Write};
use vec3::{dot_product, Vec3};

// TODO: Util function for vec3 is still incomplete
// TODO: Maybe create a new config struct to store all the settings

fn hit_sphere(center: Point3, radius: f64, ray: &ray::Ray) -> bool {
    let oc = center - ray.origin();

    let a = dot_product(ray.direction(), ray.direction());
    let b = dot_product(ray.direction(), oc) * -2.0;
    let c = dot_product(oc, oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;

    discriminant >= 0.0
}

fn ray_color(ray: &ray::Ray) -> Color {
    if hit_sphere(
        Point3 {
            element: [0.0, 0.0, -1.0],
        },
        0.5,
        ray,
    ) {
        return Color::new(1.0, 0.0, 0.0);
    }

    if hit_sphere(
        Point3 {
            element: [0.0, 0.5, -1.0],
        },
        0.5,
        ray,
    ) {
        return Color::new(0.0, 1.0, 0.0);
    }

    if hit_sphere(
        Point3 {
            element: [0.0, -0.5, -1.0],
        },
        0.5,
        ray,
    ) {
        return Color::new(0.0, 0.0, 1.0);
    }

    let unit_direction = vec3::unit_vector(ray.direction());
    let a = (unit_direction.y() + 1.0) * 0.5;

    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    SimpleLogger::new().init().unwrap();

    // Image
    const IMAGE_WIDTH: u32 = 720;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const MAX_COLOR: u32 = 255;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    const FOCAL_LENGTH: f64 = 1.0;
    let camera_center: Point3 = Point3 {
        element: [0.0, 0.0, 0.0],
    };

    // Calculate vectors across U and V. We're setting Y to minus because
    // PPM renders image from top to bottom
    let viewport_u = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    //Calculate pixel delta between U and V
    let pixel_delta_u = viewport_u / IMAGE_WIDTH.into();
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT.into();

    //Calculate location for upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    //Headers
    let mut file = File::create("image.ppm").unwrap();
    file.write_all("P3\n".as_bytes()).unwrap();
    file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
        .unwrap();
    file.write_all(format!("{}\n", MAX_COLOR).as_bytes())
        .unwrap();

    //Pixels
    info!("Starting to write pixels into the image file");
    let progress_bar = ProgressBar::new(IMAGE_HEIGHT.into());

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let pixel_center = pixel_loc + (pixel_delta_u * x.into()) + (pixel_delta_v * y.into());
            let ray_direction = pixel_center - camera_center;

            let ray = ray::Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            write_color(&mut file, pixel_color);
        }

        progress_bar.inc(1);
    }

    progress_bar.finish();
}
