use exif::{Exif, In, Tag, Value};
use std::{env, fs::OpenOptions};

extern crate exif;
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

        println!("orientation is {}", get_orientation_value(&exif));
    }
}

pub fn get_orientation_value(exif: &Exif) -> u16 {
    let f = exif.get_field(Tag::Orientation, In::PRIMARY).unwrap();
    match &f.value {
        Value::Short(v) => v[0],
        _ => 0u16,
    }
}
