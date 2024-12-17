use std::{fs::File, io::Write};

use crate::{shapes::interval::Interval, vec3};

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
}

pub fn write_color(file: &mut File, color: Color) -> std::io::Result<()> {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let intensity = Interval::new(0.000, 0.999);

    let rbyte = (intensity.clamp(r) * 255.0) as u64;
    let gbyte = (intensity.clamp(g) * 255.0) as u64;
    let bbyte = (intensity.clamp(b) * 255.0) as u64;

    file.write_all(format!("{} {} {}\n", rbyte, gbyte, bbyte).as_bytes())
}
