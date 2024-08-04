extern crate logger;

use crate::mqerror::{self, MqError};

use stomp_rs::{
    client::{Client, ClientBuilder},
    protocol::{frame::Subscribe, Frame, ServerCommand},
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

// structures and traits
#[derive(Clone)]
pub struct InternalMQ {
    host: String,
    port: u16,
    mqname: String,
}

impl InternalMQ {
    pub fn new(host: String, _port: u16, mqname: String) -> Self {
        Self {
            host,
            port: 61613,
            mqname,
        }
    }

    pub fn get_queue_name(&self) -> String {
        format!("/queue/{}", self.mqname)
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub async fn connect(&mut self) -> mqerror::Result<Box<Client>> {
        let client = Client::connect(ClientBuilder::new(self.get_address()))
            .await
            .map_err(|e| mqerror::MqError::TextErr(e.to_string()))?;

        Ok(Box::new(client))
    }

    pub async fn subscribe(
        &self,
        client: &Client,
    ) -> mqerror::Result<Receiver<Frame<ServerCommand>>> {
        let (sender, receiver): (Sender<Frame<ServerCommand>>, Receiver<Frame<ServerCommand>>) =
            channel(16);
        let subs = Subscribe::new_with_random_id(self.get_queue_name());

        client
            .subscribe(subs, sender)
            .await
            .map_err(|e| mqerror::MqError::TextErr(e.to_string()))?;

        Ok(receiver)
    }

    pub async fn receive(
        &self,
        receiver: &mut Receiver<Frame<ServerCommand>>,
    ) -> mqerror::Result<()> {
        let opt_frame = receiver
            .recv()
            .await
            .ok_or(|_| mqerror::MqError::TextErr("error in open frame".into()))
            .map_err(|e| e("error in open frame"))?;

        let body_str = opt_frame.body.as_bytes();

        println!("frame with body has received : {:?}", body_str);

        Ok(())
    }

    pub async fn send(&self, payload: String, client: &Client) -> mqerror::Result<()> {
        let send = stomp_rs::protocol::frame::Send::new(self.get_queue_name()).body(payload);
        client
            .send(send)
            .await
            .map_err(|e| mqerror::MqError::TextErr(e.to_string()))?;
        Ok(())
    }
}

/////////////////////////////////
// public functions
pub async fn init_session<'a>(
    mq_address: String,
    mqname: String,
) -> mqerror::Result<Box<InternalMQ>> {
    //create session
    let purl = url::Url::parse(&mq_address)
        .map_err(|e: url::ParseError| MqError::TextErr(e.to_string()))?;
    let host = purl
        .host_str()
        .ok_or(MqError::TextErr("error in get host from address".into()))?;
    let port = purl
        .port()
        .ok_or(MqError::TextErr("error in get port from address".into()))?;

    //end
    let instance = InternalMQ::new(host.to_string(), port, mqname);
    Ok(Box::new(instance))
}
