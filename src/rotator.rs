extern crate exif;
extern crate image;

use exif::{Exif, In, Tag, Value};
use image::ImageFormat;
use std::io::Seek;
use std::{fs::OpenOptions, path::Path};

pub fn run_rotation(path: String) {
    let path = &Path::new(&path);
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

    let rotated_image = match image::load(bufreader, ImageFormat::Jpeg) {
        Ok(image) => match orientation {
            1 | 2 | 4 | 5 | 7 => {
                println!("orientation looks cool already!");
                None
            }
            8 => {
                println!("rotating 270 degrees ...");
                Some(image.rotate270())
            }
            3 => {
                println!("rotating 180 degrees ...");
                Some(image.rotate180())
            }
            6 => {
                println!("rotating 90 degrees ...");
                Some(image.rotate90())
            }
            _ => {
                println!("invalid orientation found :(");
                None
            }
        },
        _ => {
            println!("error loading image. this tool works with valid jpeg images.");
            None
        }
    };

    if let Some(image) = rotated_image {
        let path = format!(
            "./images-test/images/rotated_{}",
            path.file_name().unwrap().to_str().unwrap()
        );
        let path = Path::new(&path);

        println!("creating a file with path {:?}", path);
        let _ = std::fs::File::create(&path).unwrap();

        match image.save(path) {
            Ok(_) => println!("saved file!"),
            Err(e) => println!("error saving file {}", e),
        }
    } else {
        println!("no processing done on current file.");
    }
}

fn get_orientation_value(exif: &Exif) -> u16 {
    let f = exif.get_field(Tag::Orientation, In::PRIMARY).unwrap();
    match &f.value {
        Value::Short(v) => v[0],
        _ => 0u16,
    }
}
