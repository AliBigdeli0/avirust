use tokio::{sync::mpsc, task::JoinHandle};

use crate::app_errors::AppResult;

pub fn start_memory_writer(mut rx: mpsc::Receiver<String>) -> JoinHandle<AppResult<()>> {
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            logger::debug(&format!("{msg}"));
        }

        Ok(())
    })
}
