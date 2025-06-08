extern crate logger;

use core::panic;
use std::{future::Future, pin::Pin};

use crate::mqerror::{self, MqError};

use stomp_rs::{
    client::{Client, ClientBuilder},
    protocol::{frame::Subscribe, Frame, ServerCommand},
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

type MQBytes = Vec<u8>;

#[derive(Clone)]
pub enum TypeOfMQ {
    Stomp = 0,
    Lappin = 1,
}

// structures and traits

pub trait MQData: Send {
    fn get(self) -> MQBytes;
    fn set(&mut self, bytes: &mut MQBytes);
}

pub trait MQActions: Send {
    fn connect_to_mq(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = mqerror::Result<Box<dyn MQData>>> + Send + '_>>;
    // fn send(&self) -> mqerror::Result<&dyn MQData>;
    // fn recieve(&self) -> mqerror::Result<&dyn MQData>;
}

pub struct InternalMQData {
    data: MQBytes,
}

impl InternalMQData {
    pub fn new(data: MQBytes) -> Self {
        Self { data }
    }
}

impl MQData for InternalMQData {
    fn get(self) -> MQBytes {
        self.data
    }

    fn set(&mut self, bytes: &mut MQBytes) {
        self.data = bytes.to_vec();
    }
}

pub struct InternalMQStomp {
    host: String,
    port: u16,
    mqname: String,
    client: Option<Box<Client>>,
}

impl InternalMQStomp {
    pub fn new(host: String, _port: u16, mqname: String) -> Self {
        Self {
            host,
            port: 61613,
            mqname,
            client: None,
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

impl MQActions for InternalMQStomp {
    fn connect_to_mq(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = mqerror::Result<Box<dyn MQData>>> + Send + '_>> {
        Box::pin(async move {
            let conn: MQBytes = Vec::<u8>::new();

            let client = self.connect().await?;

            self.client = Some(client);

            let internal_connection = InternalMQData::new(conn);

            let internal_connection_box: Box<dyn MQData> = Box::new(internal_connection);
            Ok(internal_connection_box)
        })
    }

    // fn send(&self) -> mqerror::Result<&dyn MQData> {
    //     todo!()
    // }

    // fn recieve(&self) -> mqerror::Result<&dyn MQData> {
    //     todo!()
    // }
}

/////////////////////////////////
// public functions
pub async fn init_session<'a>(
    mq_address: String,
    mqname: String,
    type_of_mq: TypeOfMQ,
) -> mqerror::Result<Box<dyn MQActions>> {
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
    match type_of_mq {
        TypeOfMQ::Lappin => panic!("This part is not implemented yet!"),
        TypeOfMQ::Stomp => {
            let instance = InternalMQStomp::new(host.to_string(), port, mqname);
            Ok(Box::new(instance))
        }
    }
}
