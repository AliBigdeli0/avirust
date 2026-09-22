use tokio::{sync::mpsc, task::JoinHandle};

use crate::app_errors::AppResult;

pub fn start_redis_writer(mut rx: mpsc::Receiver<String>) -> JoinHandle<AppResult<()>> {
    tokio::spawn(async move {
        while let Some(_) = rx.recv().await {
            // push_to_redis(&message).await?;
        }

        println!("Redis channel drained, worker stopping");

        Ok(())
    })
}
