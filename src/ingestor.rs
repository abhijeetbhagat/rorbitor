use crate::rotator::JPEGRotator;
use async_std::task;
use futures::future::join_all;
use std::io::Error;
pub struct Ingestor;

impl Ingestor {
    /// Accepts a list of jpeg paths and spawns off async tasks to
    /// perform rotations on them. One task handles one image rotation.
    pub async fn start(files: Vec<String>) {
        let mut futures_list = vec![];
        for path in files {
            futures_list.push(task::spawn(async {
                let rotator = JPEGRotator::new(path)?;
                if let Err(e) = rotator.rotate().await {
                    println!("an error occurred - {}", e);
                }
                Ok::<(), Error>(())
            }));
        }

        println!("waiting for all rotations to complete ...");
        join_all(futures_list).await;
    }
}
