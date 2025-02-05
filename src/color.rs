use std::{fs::File, io::Write};

use crate::{shapes::interval::Interval, util::{random_double, random_double_with_range}, vec3};

pub type Color = vec3::Vec3;

impl Color {
    pub const WHITE: Color = Color {
        element: [1.0, 1.0, 1.0],
    };
    pub const BLACK: Color = Color {
        element: [0.0, 0.0, 0.0],
    };
    pub const RED: Color = Color {
        element: [1.0, 0.0, 0.0],
    };
    pub const GREEN: Color = Color {
        element: [0.0, 1.0, 0.0],
    };
    pub const BLUE: Color = Color {
        element: [0.0, 0.0, 1.0],
    };

    pub const LIGHT_BLUE: Color = Color {
        element: [0.5, 0.7, 1.0],
    };

    pub fn from_hex(_hex: String) -> Color {
        todo!()
    }
    
    pub fn random() -> Color {
        Color {
            element: [
                random_double_with_range(0.0, 1.0),
                random_double_with_range(0.0, 1.0),
                random_double_with_range(0.0, 1.0),
            ],
        }
}
}

fn linear_to_gamma(linear_input: f64) -> f64 {
    if linear_input > 0.0 {
        return linear_input.sqrt();
    }

    return 0.0;
}

pub fn write_color(file: &mut File, color: Color) -> std::io::Result<()> {
    // Get the color value and apply gamma transformation
    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());

    let intensity = Interval::new(0.000, 0.999);

    let rbyte = (intensity.clamp(r) * 255.0) as u64;
    let gbyte = (intensity.clamp(g) * 255.0) as u64;
    let bbyte = (intensity.clamp(b) * 255.0) as u64;

    file.write_all(format!("{} {} {}\n", rbyte, gbyte, bbyte).as_bytes())
}
