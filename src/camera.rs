use core::f64;
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
    util::{degrees_to_radians, random_double},
    vec3::{random_unit_disk, Vec3},
};

struct FrameBasis {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}
pub struct Camera {
    pub image_width: u32,
    pub aspect_ratio: f64,
    image_height: u32,
    max_color: u32,

    pub samples_per_pixel: u64,
    pub max_ray_depth: i64,
    camera_center: Point3,
    focal_point: f64,
    viewport_height: f64,
    viewport_width: f64,
    pixel_samples_scale: f64,
    pub vertical_fov: f64,
    pub defocus_angle: f64,
    pub focus_distance: f64,

    viewport_u: Vec3,
    viewport_v: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    viewport_upper_left: Vec3,
    pixel_loc: Vec3,

    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,

    frame_basis: FrameBasis,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            image_width: 0,
            aspect_ratio: 0.0,
            image_height: 0,
            max_color: 0,

            samples_per_pixel: 0,
            max_ray_depth: 0,
            camera_center: Point3::zero(),
            focal_point: 0.0,
            viewport_height: 0.0,
            viewport_width: 0.0,
            pixel_samples_scale: 0.0,
            vertical_fov: 0.0,

            look_from: Point3::zero(),
            look_at: Point3::zero(),
            vup: Vec3::zero(),

            viewport_u: Vec3::zero(),
            viewport_v: Vec3::zero(),

            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),

            viewport_upper_left: Vec3::zero(),
            pixel_loc: Vec3::zero(),
            frame_basis: FrameBasis {
                u: Vec3::zero(),
                v: Vec3::zero(),
                w: Vec3::zero(),
            },

            defocus_angle: 0.0,
            focus_distance: 10.0,

            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
        }
    }
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u64,
        max_ray_depth: i64,
        vertical_fov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Camera {
        Camera {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            max_ray_depth,
            vertical_fov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_distance,
            ..Default::default()
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.camera_center = self.look_from;
        self.max_color = 255;

        let theta = degrees_to_radians(self.vertical_fov);
        let h = (theta / 2.0).tan();
        self.viewport_height = 2.0 * h * self.focus_distance;
        self.viewport_width =
            self.viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        // Setting frame basis for the camera coordinate frame
        self.frame_basis.w = (self.look_from - self.look_at).unit_vector();
        self.frame_basis.u = (self.vup.cross_product(self.frame_basis.w)).unit_vector();
        self.frame_basis.v = self.frame_basis.w.cross_product(self.frame_basis.u);

        self.viewport_u = self.viewport_width * self.frame_basis.u;
        self.viewport_v = self.viewport_height * -self.frame_basis.v;

        self.pixel_delta_u = self.viewport_u / self.image_width.into();
        self.pixel_delta_v = self.viewport_v / self.image_height.into();

        self.viewport_upper_left = self.camera_center
            - (self.focus_distance * self.frame_basis.w)
            - (self.viewport_u / 2.0)
            - (self.viewport_v / 2.0);
        self.pixel_loc = self.viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius =
            self.focus_distance * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.frame_basis.u * defocus_radius;
        self.defocus_disk_v = self.frame_basis.v * defocus_radius;
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
                let mut pixel_color = Color::zero();

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += Self::ray_color(&ray, world, self.max_ray_depth);
                }

                let result = write_color(&mut file, pixel_color * self.pixel_samples_scale);

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

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = Self::sample_square();

        let pixel_sample = self.pixel_loc
            + (self.pixel_delta_u * (x as f64 + offset.x()))
            + (self.pixel_delta_v * (y as f64 + offset.y()));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns random point in camera defocus disk
        let p = random_unit_disk();

        self.camera_center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(ray: &Ray, world: &dyn hittable::Hittable, max_ray_depth: i64) -> Color {
        if max_ray_depth <= 0 {
            return Color::BLACK;
        }

        let mut rec = HitRecord::new();

        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered_ray = Ray::new(Vec3::zero(), Vec3::zero());
            let mut attenuation: Color = Color::WHITE;

            if rec
                .material
                .scatter(ray, &rec, &mut attenuation, &mut scattered_ray)
            {
                return attenuation * Self::ray_color(&scattered_ray, world, max_ray_depth - 1);
            }

            return Color::BLACK;
        }

        let unit_direction = ray.direction().unit_vector();
        let a = (unit_direction.y() + 1.0) * 0.5;

        Color::WHITE * (1.0 - a) + Color::LIGHT_BLUE * a
    }
}