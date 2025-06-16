extern crate logger;

use core::panic;
use std::{future::Future, pin::Pin};

use crate::mqerror::{self, MqError};
use crate::openwire::OpenWire;

use stomp_rs::{
    client::{Client, ClientBuilder},
    protocol::{frame::Subscribe, Frame, ServerCommand},
};

use tokio::sync::mpsc::{channel, Receiver, Sender};

type MQBytes = Vec<u8>;

#[derive(Clone)]
pub enum TypeOfMQ {
    Stomp = 0x0,
    Lappin = 0xf,
    OpenWire = 0xff,
}

// structures and traits

pub trait MQData: Send {
    fn get(self) -> MQBytes;
    fn set(&mut self, bytes: &mut MQBytes);
}

pub trait MQActions<'a>: Send {
    fn connect_to_mq(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = mqerror::Result<Box<dyn MQData>>> + Send + '_>>;

    fn receive(
        &self,
        receiver_fn: Box<dyn Fn(Box<MQBytes>) + Send>,
    ) -> Pin<Box<dyn Future<Output = mqerror::Result<()>> + Send + '_>>;
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

// ##################################
// OpenWire
struct InternalOpenwire<'a> {
    address: &'a str,
    open_wire: Option<Box<OpenWire<'a>>>,
}

impl<'a> InternalOpenwire<'a> {
    pub fn new(address: &'a str) -> Self {
        Self {
            address,
            open_wire: None,
        }
    }
}

impl<'a> Drop for InternalOpenwire<'a> {
    fn drop(&mut self) {
        /* nothing */
    }
}

impl<'a> MQActions<'a> for InternalOpenwire<'a> {
    fn connect_to_mq(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = mqerror::Result<Box<dyn MQData>>> + Send + '_>> {
        let mut openwire_instance = OpenWire::new(self.address);

        Box::pin(async move {
            openwire_instance.start().await?;

            self.open_wire = Some(Box::new(openwire_instance));

            let conn: MQBytes = Vec::<u8>::new();
            let internal_connection = InternalMQData::new(conn);
            let internal_connection_box: Box<dyn MQData> = Box::new(internal_connection);
            Ok(internal_connection_box)
        })
    }

    fn receive(
        &self,
        _: Box<dyn Fn(Box<MQBytes>) + Send>,
    ) -> Pin<Box<dyn Future<Output = mqerror::Result<()>> + Send + '_>> {
        Box::pin(async move {
            logger::info("inside the receiver!!!");

            Ok(())
        })
    }
}

// ##################################
// Stomp
pub struct InternalMQStomp<'a> {
    host: String,
    port: u16,
    mqname: &'a str,
    client: Option<Box<Client>>,
}

impl<'a> InternalMQStomp<'a> {
    pub fn new(host: String, _port: u16, mqname: &'a str) -> Self {
        Self {
            host,
            port: 61616,
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

    pub async fn subscribe(&self) -> mqerror::Result<Receiver<Frame<ServerCommand>>> {
        let (sender, receiver): (Sender<Frame<ServerCommand>>, Receiver<Frame<ServerCommand>>) =
            channel(16);
        let subs = Subscribe::new_with_random_id(self.get_queue_name());

        if let Some(self_client) = &self.client {
            self_client
                .as_ref()
                .subscribe(subs, sender)
                .await
                .map_err(|e| mqerror::MqError::TextErr(e.to_string()))?;

            Ok(receiver)
        } else {
            Err(mqerror::MqError::TextErr(
                "error in subscribe into mq!".to_string(),
            ))
        }
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

    pub fn make_bytes(&self, mq_str: &'a str) -> Box<MQBytes> {
        let mq_bytes: MQBytes = String::from(mq_str).into_bytes();

        Box::new(mq_bytes)
    }
}

impl<'a> MQActions<'a> for InternalMQStomp<'a> {
    fn connect_to_mq(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = mqerror::Result<Box<dyn MQData>>> + Send + '_>> {
        Box::pin(async move {
            let client = self.connect().await?;

            self.client = Some(client);

            let conn: MQBytes = Vec::<u8>::new();
            let internal_connection = InternalMQData::new(conn);
            let internal_connection_box: Box<dyn MQData> = Box::new(internal_connection);
            Ok(internal_connection_box)
        })
    }

    fn receive(
        &self,
        receiver_fn: Box<dyn Fn(Box<MQBytes>) + Send>,
    ) -> Pin<Box<dyn Future<Output = mqerror::Result<()>> + Send + '_>> {
        Box::pin(async move {
            let mut subscriber = self.subscribe().await?;

            loop {
                // receiver.recv().await;
                let bytes = subscriber.recv().await;

                if let Some(bytes) = bytes {
                    let mq_bytes_ref = self.make_bytes(&bytes.body);
                    receiver_fn(mq_bytes_ref);
                }
            }
        })
    }
}

/////////////////////////////////
// public functions
pub async fn init_session<'a>(
    mq_address: &'a str,
    mqname: &'a str,
    type_of_mq: TypeOfMQ,
) -> mqerror::Result<Box<dyn MQActions<'a> + 'a>> {
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
        TypeOfMQ::OpenWire => {
            let openwire_instance = InternalOpenwire::new(mq_address);
            Ok(Box::new(openwire_instance))
        }
        TypeOfMQ::Stomp => {
            let instance: InternalMQStomp<'a> = InternalMQStomp::new(host.into(), port, mqname);
            Ok(Box::new(instance))
        }
    }
}
