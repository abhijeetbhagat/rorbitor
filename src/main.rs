use std::env;
pub mod rotator;

fn main() {
    for path in env::args().skip(1) {
        rotator::run_rotation(path);
    }
}
