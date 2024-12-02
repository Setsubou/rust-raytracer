pub mod vec3;
pub mod color;

use color::{write_color, Color};
use std::{fs::File, io::Write};
use simple_logger::SimpleLogger;
use log::info;

// TODO: Wanted to do a progress bar instead of spamming the console
// TODO: Util function for vec3 is still incomplete

fn main() {
    SimpleLogger::new().init().unwrap();

    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    const MAX_COLOR: u32 = 255;

    let mut file = File::create("image.ppm").unwrap();

    //Headers
    file.write("P3\n".as_bytes()).unwrap();
    file.write(String::from(format!("{} {}\n", WIDTH, HEIGHT)).as_bytes()).unwrap();
    file.write(String::from(format!("{}\n", MAX_COLOR)).as_bytes()).unwrap();


    //Pixels
    info!("Starting to write pixels into the image");

    for y in 0..HEIGHT {
        info!("{} scanlines remaining", HEIGHT - y);

        for x in 0..WIDTH {
            let pixel_color = Color::new((x as f64) / (WIDTH as f64 - 1.0), (y as f64) / (HEIGHT as f64 - 1.0), 0.0);
            write_color(&mut file, pixel_color);
        }
    }
}