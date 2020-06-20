use exif::{Exif, In, Tag, Value};
use std::io::Seek;
use std::{env, fs::OpenOptions};

extern crate exif;
extern crate image;

use image::ImageFormat;

fn main() {
    for path in env::args().skip(1) {
        let file = match OpenOptions::new().read(true).write(true).open(&path) {
            Ok(file) => file,
            Err(e) => {
                println!("error occurred during opening file: {}", e);
                return;
            }
        };
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = match exifreader.read_from_container(&mut bufreader) {
            Ok(exif) => exif,
            Err(e) => {
                println!("couldn't parse exif information: {}", e);
                return;
            }
        };
        let orientation = get_orientation_value(&exif);
        println!("orientation is {}", orientation);

        //Reset fp to the beginning; otherwise loading fails
        bufreader.seek(std::io::SeekFrom::Start(0));

        match image::load(bufreader, ImageFormat::Jpeg) {
            Ok(image) => match orientation {
                1 | 2 | 4 | 5 | 7 => {
                    println!("orientation looks cool already!");
                }
                8 => {
                    println!("rotating 270 degrees ...");
                    image.rotate270();
                }
                3 => {
                    println!("rotating 180 degrees ...");
                    image.rotate180();
                }
                6 => {
                    println!("rotating 90 degrees ...");
                    image.rotate90();
                }
                _ => {
                    println!("invalid orientation found :(");
                }
            },
            _ => {
                println!("error loading image. this tool works with valid jpeg images.");
            }
        }
    }
}

pub fn get_orientation_value(exif: &Exif) -> u16 {
    let f = exif.get_field(Tag::Orientation, In::PRIMARY).unwrap();
    match &f.value {
        Value::Short(v) => v[0],
        _ => 0u16,
    }
}
