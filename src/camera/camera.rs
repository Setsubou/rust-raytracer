use std::{fs::File, io::Write};

use indicatif::ProgressBar;
use log::info;

use crate::{
    color::{write_color, Color},
    hit_record::HitRecord,
    hittable,
    hittable_list::HittableList,
    point::Point3,
    ray::Ray,
    shapes::interval::Interval,
    vec3::{self, Vec3},
};

pub struct Camera {
    pub image_width: u32,
    pub aspect_ratio: f64,
    image_height: u32,
    max_color: u32,

    camera_center: Point3,
    focal_point: f64,
    viewport_height: f64,
    viewport_width: f64,

    viewport_u: Vec3,
    viewport_v: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    viewport_upper_left: Vec3,
    pixel_loc: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            image_width: 0,
            aspect_ratio: 0.0,
            image_height: 0,
            max_color: 0,

            camera_center: Point3::zero(),
            focal_point: 0.0,
            viewport_height: 0.0,
            viewport_width: 0.0,

            viewport_u: Vec3::zero(),
            viewport_v: Vec3::zero(),

            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),

            viewport_upper_left: Vec3::zero(),
            pixel_loc: Vec3::zero(),
        }
    }
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f64) -> Camera {
        Camera {
            image_width,
            aspect_ratio,
            ..Default::default()
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.camera_center = Point3::zero();
        self.max_color = 255;

        self.focal_point = 1.0;
        self.viewport_height = 2.0;
        self.viewport_width =
            self.viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.viewport_u = Vec3::new(self.viewport_width, 0.0, 0.0);
        self.viewport_v = Vec3::new(0.0, -self.viewport_height, 0.0);

        self.pixel_delta_u = self.viewport_u / self.image_width.into();
        self.pixel_delta_v = self.viewport_v / self.image_height.into();

        self.viewport_upper_left = self.camera_center
            - Vec3::new(0.0, 0.0, self.focal_point)
            - (self.viewport_u / 2.0)
            - (self.viewport_v / 2.0);
        self.pixel_loc = self.viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }   

    pub fn render(&mut self, world: &mut HittableList) {
        self.initialize();

        let mut file = File::create("image.ppm").unwrap();
        file.write_all("P3\n".as_bytes()).unwrap();
        file.write_all(format!("{} {}\n", self.image_width, self.image_height).as_bytes())
            .unwrap();
        file.write_all(format!("{}\n", self.max_color).as_bytes())
            .unwrap();

        info!("Starting to write pixels into the image file");
        let progress_bar = ProgressBar::new(self.image_height.into());

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let pixel_center = self.pixel_loc
                    + (self.pixel_delta_u * x.into())
                    + (self.pixel_delta_v * y.into());
                let ray_direction = pixel_center - self.camera_center;
                let ray = Ray::new(self.camera_center, ray_direction);

                let pixel_color = Self::ray_color(&ray, world);

                let result = write_color(&mut file, pixel_color);

                match result {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error rendering image: {}", e);
                        std::process::exit(1);
                    }
                }
            }

            progress_bar.inc(1);
        }

        progress_bar.finish();
    }

    fn ray_color(ray: &Ray, world: &dyn hittable::Hittable) -> Color {
        let mut rec = HitRecord::new();

        if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return (rec.normal + Color::WHITE) * 0.5;
        }

        let unit_direction = vec3::unit_vector(&ray.direction());
        let a = (unit_direction.y() + 1.0) * 0.5;

        Color::WHITE * (1.0 - a) + Color::LIGHT_BLUE * a
    }
}
