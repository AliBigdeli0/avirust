use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use url::Url;

use crate::{
    mqerror::{self, MqError},
    openwire_commands::OpenWireCommandTypes,
};

pub struct OpenWire<'a> {
    address: &'a str,
    mq_name: &'a str,
    stream: Option<Box<TcpStream>>,
}

type MQResult<T> = mqerror::Result<T>;

impl<'a> OpenWire<'a> {
    pub fn new(address: &'a str, mq_name: &'a str) -> Self {
        Self {
            address,
            mq_name,
            stream: None,
        }
    }

    pub async fn start(&mut self) -> mqerror::Result<()> {
        let (address_str, port) = self.parse_url()?;
        let stream = TcpStream::connect(format!("{address_str}:{port}"))
            .await
            .map_err(|e| mqerror::MqError::TextErr(e.to_string()))?;

        self.stream = Some(Box::new(stream));

        logger::info(&format!(
            "Openwire tcp connection established! The MQ name is: {}",
            self.mq_name
        ));

        let mut stream = self.stream.take().unwrap();

        loop {
            if !self.check_connection(&mut stream).await? {
                break;
            }

            logger::info("continue to work!!!!");
        }

        Ok(())
    }

    async fn check_connection(&mut self, stream: &mut Box<TcpStream>) -> MQResult<bool> {
        match stream.writable().await {
            Ok(_) => {
                // logger::info("everything connection is available!!!");
                // nothing : connection is OK
            }
            Err(err) => {
                logger::error(&format!(
                    "The connection is not writable anymore, goint to reset it. The error is: {}",
                    err
                ));

                return Ok(false);
            }
        }

        let socket_content = self
            .read_from_socket(
                stream,
                OpenWireCommandTypes::ConnectionInfo,
                [0x00, 0x00, 0x00, 0x01],
            )
            .await?;

        Ok(socket_content.starts_with("RActiveMQ"))
    }

    async fn read_from_socket(
        &mut self,
        stream: &mut Box<TcpStream>,
        mq_type: OpenWireCommandTypes,
        command_id: [u8; 4],
    ) -> MQResult<String> {
        let mq_type_u8 = mq_type as u8;

        logger::info(&format!(
            "The mq type : {} and command lenght: {}",
            mq_type_u8,
            command_id.len()
        ));
        let openwire_packet: [u8; 5] = [
            mq_type_u8,
            command_id[0],
            command_id[1],
            command_id[2],
            command_id[3],
        ];

        if let Err(e) = stream.write(&openwire_packet).await {
            logger::error(&format!("Failed to check the openwire connection : {}", e));
            return Ok("".into());
        }

        let mut buf = [0u8; 1024];
        let mut packet_response_vec = Vec::new();

        loop {
            let n = match stream.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    logger::error(&format!("Read error {}", e));
                    break;
                }
            };

            packet_response_vec.extend_from_slice(&buf[..n]);
        }

        if !packet_response_vec.is_empty() {
            let recieved_message = String::from_utf8(packet_response_vec).map_err(|e| {
                MqError::TextErr(format!(
                    "Error in converting the response packet to string, {}",
                    e
                ))
            })?;

            logger::info(&format!("The message is {}", recieved_message));
            // logger::info(&format!("the message size: {}", recieved_message.len()));
            return Ok(recieved_message);
        }

        Ok("".into())
    }

    fn parse_url(&self) -> mqerror::Result<(String, u16)> {
        let url = Url::parse(self.address).map_err(|e| mqerror::MqError::TextErr(e.to_string()))?;

        let host: String = url
            .host_str()
            .ok_or(MqError::TextErr("Error in parsing host name.".to_string()))?
            .to_string();

        let port = url
            .port_or_known_default()
            .ok_or(MqError::TextErr("Error in parsing port number.".into()))?;

        Ok((host, port))
    }

    pub fn close(&mut self) {
        let _ = std::mem::take(&mut self.stream);
    }
}

impl<'a> Drop for OpenWire<'a> {
    fn drop(&mut self) {
        logger::info("Dropping the openwire object!".into());
        self.close()
    }
}
