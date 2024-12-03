pub mod vec3;
pub mod color;
pub mod point;
pub mod ray;

use color::{write_color, Color};
use indicatif::ProgressBar;
use point::Point3;
use vec3::Vec3;
use std::{fs::File, io::Write};
use simple_logger::SimpleLogger;
use log::info;

// TODO: Util function for vec3 is still incomplete

fn ray_color(ray: &ray::Ray) -> Color {
    let unit_direction = vec3::unit_vector(ray.direction());
    let a = (unit_direction.y() + 1.0) * 0.5;

    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    SimpleLogger::new().init().unwrap();

    // Image
    const IMAGE_WIDTH: u32 = 360;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const MAX_COLOR: u32 = 255;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    const FOCAL_LENGTH: f64 = 1.0;
    let camera_center: Point3 = Point3{element: [0.0, 0.0, 0.0]};

    // Calculate vectors across U and V. We're Minus Y since we're using right hand coordinate, 
    // and yet PPM renders image from top to bottom
    let viewport_U = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_V = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    //Calculate pixel delta between U and V
    let pixel_delta_U = viewport_U / IMAGE_HEIGHT.into();
    let pixel_delta_V = viewport_V / IMAGE_WIDTH.into();

    //Calculate location for upper left pixel
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - (viewport_U / 2.0) - (viewport_V / 2.0);
    let pixel_loc = viewport_upper_left + (pixel_delta_U + pixel_delta_V) * 0.5;

    //Headers
    let mut file = File::create("image.ppm").unwrap();
    file.write("P3\n".as_bytes()).unwrap();
    file.write(String::from(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT)).as_bytes()).unwrap();
    file.write(String::from(format!("{}\n", MAX_COLOR)).as_bytes()).unwrap();


    //Pixels
    info!("Starting to write pixels into the image file");
    let progress_bar = ProgressBar::new(IMAGE_HEIGHT.into());

    for y in 0..IMAGE_HEIGHT {

        for x in 0..IMAGE_WIDTH {
            let pixel_center = pixel_loc + (pixel_delta_U * x.into()) + (pixel_delta_V * y.into());
            let ray_direction = pixel_center - camera_center;

            let ray = ray::Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            write_color(&mut file, pixel_color);
        }

        progress_bar.inc(1);
    }

    progress_bar.finish();
}