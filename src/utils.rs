use exif::{Exif, In, Tag, Value};
use std::fs::File;
use std::io::Seek;

/// Reads EXIF data from the given file and returns orientation value
/// as an unsigned short.
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

/// Gets orientation value as an unsigned short from an Exif reference
pub fn get_orientation_value(exif: &Exif) -> u16 {
    let f = exif.get_field(Tag::Orientation, In::PRIMARY).unwrap();
    match &f.value {
        Value::Short(v) => v[0],
        _ => 0u16,
    }
}

/// Returns a vector of bytes representing the whole EXIF data
/// with updated orientation value of 1.
/// This should be really handled by the image library but ...
pub fn get_orientation_fixed_exif(exif: &Exif) -> Vec<u8> {
    let mut buf = vec![0xffu8, 0xe1, 0x0, exif.buf().len() as u8];
    buf.extend_from_slice(b"EXIF\0\0");
    buf.extend_from_slice(exif.buf());
    let orientation_offset = buf[16..]
        .chunks(2)
        .position(|a| a[0] == 0x1 && a[1] == 0x12)
        .unwrap();
    //skip past exif 'header' + offset of the orientation marker bytes + offset
    //of the actual orientation value which seems to be 9 bytes and set
    //orientation to 1
    buf[16 + orientation_offset + 9] = 1;
    buf
}
