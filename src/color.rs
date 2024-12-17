use std::{fs::File, io::Write};

use crate::vec3;

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
    let r = (255.00 * color.x()).round() as u32;
    let g = (255.00 * color.y()).round() as u32;
    let b = (255.00 * color.z()).round() as u32;

    file.write_all(format!("{} {} {}\n", r, g, b).as_bytes())
}
