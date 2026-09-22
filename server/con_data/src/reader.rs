use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

// internal
use crate::app_errors::AppResult;
use entity::model::db::Well;

pub fn start_mq_reader(
    tx: mpsc::Sender<String>,
    cancel: CancellationToken,
    well_data: Arc<Well>,
) -> JoinHandle<AppResult<()>> {
    tokio::spawn(async move {
        let mut stomp_conn = mq::mq::StompMQ::new(well_data)?;

        stomp_conn.connect().await?;
        stomp_conn.subscribe().await?;

        let run_result = stomp_conn.run(tx, cancel).await;

        let stomp_result = stomp_conn.stop().await;

        run_result?;
        stomp_result?;

        Ok(())
    })
}
