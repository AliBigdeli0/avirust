use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::app_errors::AppResult;

pub fn start_activemq_reader(
    _: mpsc::Sender<String>,
    cancel: CancellationToken,
) -> JoinHandle<AppResult<()>> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    println!("ActiveMQ reader stopping...");
                    break;
                }

                // result = receive_from_activemq() => {
                //     let message = result?;

                //     tx.send(message)
                //         .await
                //         .map_err(|_| {
                //             AppError::Text(
                //                 "Redis channel closed".into()
                //             )
                //         })?;
                // }
            }
        }

        Ok(())
    })
}
