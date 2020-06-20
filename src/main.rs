extern crate async_std;

pub mod ingestor;
pub mod rotator;
pub mod utils;

use ingestor::Ingestor;
use std::env;
use std::time::Instant;

#[async_std::main]
async fn main() {
    let now = Instant::now();
    Ingestor::start(env::args().skip(1).collect()).await;
    println!("time taken - {} secs", now.elapsed().as_secs());
}
