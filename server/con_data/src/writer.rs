use std::sync::Arc;

use common::env_helper::EnvHelper;
use entity::model::db::Well;
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;

use crate::app_errors::AppResult;

pub fn start_memory_writer(
    mut rx: mpsc::Receiver<String>,
    cancel: CancellationToken,
    well_data: Arc<Well>,
) -> JoinHandle<AppResult<()>> {
    tokio::spawn(async move {
        let mut stomp_conn = mq::mq::StompMQ::new_with_address(
            well_data.clone(),
            EnvHelper::get_amq_stomp_address(),
        )?;
        let mut msg_counter: u128 = 1;

        stomp_conn.connect().await?;

        let run_result: AppResult<()> = loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    logger::warn(
                        "STOMP writer cancelled!"
                    );

                    break Ok(());
                }

                message = rx.recv() => {
                    match message {
                        Some(body) => {
                            stomp_conn.push_to_bus(&body).await?;

                            logger::debug(format!("#{} and pushed to '{}'",msg_counter, well_data.clone().get_bus_name()).as_str());
                            logger::debug(body.as_str());
                            logger::debug("======================");
                            msg_counter += 1;
                        }

                        None => {
                            logger::error("DANGER ==> All senders are dropped");
                            break Ok(());
                        }
                    }
                }
            }
        };

        let stop_result = stomp_conn.stop().await;

        run_result?;
        stop_result?;

        Ok(())
    })
}
