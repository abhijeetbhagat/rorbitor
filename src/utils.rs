use exif::{Exif, In, Tag, Value};
use std::fs::File;
use std::io::Seek;

pub fn get_exif(file: &File) -> Result<Exif, String> {
    let orientation = {
        let mut bufreader = std::io::BufReader::new(file);
        let exifreader = exif::Reader::new();
        let exif = match exifreader.read_from_container(&mut bufreader) {
            Ok(exif) => exif,
            Err(e) => {
                return Err(format!("couldn't parse exif information: {}", e));
            }
        };

        //Reset fp to the beginning; otherwise loading fails
        match bufreader.seek(std::io::SeekFrom::Start(0)) {
            Ok(offset) => println!("new position after seek {}", offset),
            _ => println!("error seeking"),
        }
        exif
    };

    Ok(orientation)
}

pub fn get_orientation_value(exif: &Exif) -> u16 {
    let f = exif.get_field(Tag::Orientation, In::PRIMARY).unwrap();
    match &f.value {
        Value::Short(v) => v[0],
        _ => 0u16,
    }
}
