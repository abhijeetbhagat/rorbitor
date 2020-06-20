extern crate exif;
extern crate image;

use crate::utils;
use exif::{Exif, In, Tag, Value};
use image::{DynamicImage, ImageFormat};
use std::io::{Error, Seek};
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

trait Rotator {
    fn rotate(&mut self);
}
//use exif
pub struct JPEGRotator {
    path: String,
    file: File,
}

impl JPEGRotator {
    pub fn new(path: String) -> Result<Self, Error> {
        let file = OpenOptions::new().read(true).write(true).open(&path)?;

        Ok(JPEGRotator { path, file })
    }

    pub async fn rotate(&self) -> Result<(), String> {
        let exif = utils::get_exif(&self.file)?;
        let orientation = utils::get_orientation_value(&exif);
        println!("orientation is {}", orientation);

        let bufreader = std::io::BufReader::new(&self.file);

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
            self.save(image);
        }

        Ok(())
    }

    fn save(&self, image: DynamicImage) {
        println!("image buf starts with {:?}", &image.to_bytes()[..10]);

        match image.save(&self.path) {
            Ok(_) => {
                println!("{}: saved file!", self.path);
            }
            Err(e) => println!("{}: error saving file {}", e, self.path),
        }
    }
}
