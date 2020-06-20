extern crate image;

use crate::utils;

use image::{DynamicImage, ImageFormat};
use std::fs::{File, OpenOptions};
use std::io::Error;

trait Rotator {
    fn rotate(&mut self);
}
//use exif
pub struct JPEGRotator {
    path: String,
    file: File,
}

impl JPEGRotator {
    /// Returns a new JPEG rotator if the image file is successfully opened.
    /// Otherwise, an Error
    pub fn new(path: String) -> Result<Self, Error> {
        let file = OpenOptions::new().read(true).write(true).open(&path)?;

        Ok(JPEGRotator { path, file })
    }

    /// Performs rotation on the internal image handle depending on
    /// the orientation value.
    pub async fn rotate(&self) -> Result<(), String> {
        println!("processing file: {}", self.path);
        let exif = utils::get_exif(&self.file)?;
        let orientation = utils::get_orientation_value(&exif);
        println!("{}: orientation is {}", self.path, orientation);

        if let Some(image) = self.perform_rotation(orientation) {
            let _exif = utils::get_orientation_fixed_exif(&exif);
            //TODO abhi: find some way to insert EXIF data after the JFIF data
            self.save(image);
        }

        Ok(())
    }

    /// Performs rotation based on the orientation value
    fn perform_rotation(&self, orientation: u16) -> Option<DynamicImage> {
        let bufreader = std::io::BufReader::new(&self.file);

        match image::load(bufreader, ImageFormat::Jpeg) {
            Ok(image) => match orientation {
                1 | 2 | 4 | 5 | 7 => {
                    println!("{}: orientation looks cool already!", self.path);
                    None
                }
                8 => {
                    println!("{}: rotating 270 degrees ...", self.path);
                    Some(image.rotate270())
                }
                3 => {
                    println!("{}: rotating 180 degrees ...", self.path);
                    Some(image.rotate180())
                }
                6 => {
                    println!("{}: rotating 90 degrees ...", self.path);
                    Some(image.rotate90())
                }
                _ => {
                    println!("{}: invalid orientation found :(", self.path);
                    None
                }
            },
            _ => {
                println!(
                    "{}: error loading image. this tool works with valid jpeg images.",
                    self.path
                );
                None
            }
        }
    }

    /// Saves the rotated image to the same file
    fn save(&self, image: DynamicImage) {
        match image.save(&self.path) {
            Ok(_) => {
                println!("{}: saved file!", self.path);
            }
            Err(e) => println!("{}: error saving file {}", e, self.path),
        }
    }
}
