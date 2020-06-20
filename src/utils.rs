use exif::{Exif, In, Tag, Value};

pub fn get_orientation_value(exif: &Exif) -> u16 {
    let f = exif.get_field(Tag::Orientation, In::PRIMARY).unwrap();
    match &f.value {
        Value::Short(v) => v[0],
        _ => 0u16,
    }
}
