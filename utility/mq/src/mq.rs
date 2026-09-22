use futures::StreamExt;
use iridium_stomp::{AckMode, Connection, Frame, Heartbeat, Subscription};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

// internal
use common::{
    constants::STOMP_PORT,
    env_helper::EnvHelper,
    net_helper::{get_stomp_address, replace_port},
};
use entity::model::db::Well;

use crate::mqerror::MQResult;

const SEND_HEARTBEAT_MS: u32 = 10_000;
const RECEIVE_HEARTBEAT_MS: u32 = 10_000;

pub struct StompMQ {
    client_addr: String,
    well_data: Arc<Well>,

    connection: Option<Connection>,
    subscription: Option<Subscription>,
}

impl StompMQ {
    pub fn new_stomp(well_data: Arc<Well>) -> MQResult<StompMQ> {
        let converted_address = replace_port(well_data.get_server_mq_address(), STOMP_PORT)?;

        StompMQ::new_with_address(well_data, converted_address)
    }

    pub fn new_with_address(well_data: Arc<Well>, addr: String) -> MQResult<StompMQ> {
        let stomp_address = get_stomp_address(&addr)?;

        logger::info(format!("MQ initiated with the address: {}", stomp_address).as_str());

        Ok(StompMQ {
            client_addr: stomp_address,
            well_data: well_data,

            connection: None,
            subscription: None,
        })
    }

    pub async fn connect(&mut self) -> MQResult<()> {
        logger::info("Going to start mq...");
        let username = EnvHelper::get_stomp_username();
        let password = EnvHelper::get_stomp_password();

        let heartbeat = Heartbeat::new(SEND_HEARTBEAT_MS, RECEIVE_HEARTBEAT_MS);

        let connection = Connection::connect(
            self.client_addr.as_str(),
            username.as_str(),
            password.as_str(),
            &heartbeat.to_string(),
        )
        .await?;

        logger::info(format!("Activemq connected to {}", self.client_addr).as_str());

        self.connection = Some(connection);
        Ok(())
    }

    pub async fn subscribe(&mut self) -> MQResult<()> {
        if let Some(connection) = self.connection.as_ref() {
            let subscription = connection
                .subscribe(
                    &self.well_data.get_server_mq_name(),
                    AckMode::ClientIndividual,
                )
                .await?;

            logger::info(
                format!(
                    "Activemq is subscribed to {}:{}",
                    self.client_addr,
                    self.well_data.get_server_mq_name()
                )
                .as_str(),
            );

            self.subscription = Some(subscription);
        } else {
            logger::error("Invalid connection for subscription.");
        }
        Ok(())
    }
    pub async fn stop(&mut self) -> MQResult<()> {
        self.subscription.take();

        if let Some(mq_con) = self.connection.take() {
            mq_con.close().await?;
        }

        logger::warn("WARNING: mq is stopped!");
        Ok(())
    }

    pub async fn run(
        &mut self,
        tx: mpsc::Sender<String>,
        cancel: CancellationToken,
    ) -> MQResult<()> {
        if let Some(mut subscription) = self.subscription.take() {
            loop {
                let frame = tokio::select! {
                    _ = cancel.cancelled() => {
                        logger::warn("WARNING ==> Stomp reader is cancelled");
                        break;
                    }

                    frame = subscription.next() => {
                        match frame {
                            Some(frame) => frame,
                            None => {
                                logger::info("The received frame is empty (None)!");
                                break;
                            }
                        }
                    }
                };

                let body = String::from_utf8_lossy(&frame.body).into_owned();

                tokio::select! {
                    _ = cancel.cancelled() => {
                        logger::warn("WARNING ==> Stomp reader cancelled while sending");
                        break;
                    }

                    result = tx.send(body) => {
                        result?;
                    }
                }

                if let Some(message_id) = frame.get_header("message-id") {
                    subscription.ack(message_id).await?;
                }
            }

            self.subscription = Some(subscription);
        }
        Ok(())
    }

    pub async fn push_to_bus(&mut self, body: &str) -> MQResult<()> {
        if let Some(connection) = self.connection.as_ref() {
            let frame = Frame::new("SEND")
                .header("destination", self.well_data.get_topic_name())
                .header("content-type", "text/plain")
                .header("persistent", "true")
                .set_body(body.as_bytes());

            connection.send_frame(frame).await?;
        }
        Ok(())
    }
}
