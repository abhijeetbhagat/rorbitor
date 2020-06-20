extern crate async_std;
pub mod rotator;
pub mod utils;

use async_std::task;
use futures::future::join_all;
use rotator::JPEGRotator;
use std::env;
use std::time::Instant;

#[async_std::main]
async fn main() {
    let mut futures_list = vec![];
    let now = Instant::now();
    for path in env::args().skip(1) {
        futures_list.push(task::spawn(async move {
            JPEGRotator::run_rotation(path.clone()).await;
        }));
    }

    println!("waiting for all rotations to complete ...");
    join_all(futures_list).await;
    println!("time taken - {}", now.elapsed().as_secs());
}
