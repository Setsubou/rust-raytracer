use std::{fs::File, io::Write};

use crate::vec3;

pub type Color = vec3::Vec3;

impl Color {
    pub fn from_hex(_hex: String) -> Color {
        todo!()
    }
}

pub fn write_color(file: &mut File, color: Color) {
    let r = (255.00 * color.x()).trunc() as u32;
    let g = (255.00 * color.y()).trunc() as u32;
    let b = (255.00 * color.z()).trunc() as u32;

    file.write_all(format!("{} {} {}\n", r, g, b).as_bytes())
        .unwrap();
}
