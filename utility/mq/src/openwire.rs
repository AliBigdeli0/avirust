use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{sleep, Duration},
};
use url::Url;

use crate::{
    mqerror::{self, MqError},
    openwire_commands::OpenWireCommandTypes,
};

// ###############################################
// global values

type MQResult<T> = mqerror::Result<T>;

const ACTIVEMQ_HEADER: &str = "\0\0\x01R\x01ActiveMQ";
const SOCKET_BUFFER_SIZE: usize = 1024;
const WIRE_FORMAT_INFO: &[u8] = &[
    0x0A, // Data type = WireFormatInfo
    0x00, 0x00, 0x00, 0x0A, // Version = 10 (int)
    // 6 booleans (tightEncoding, stackTrace, tcpNoDelay, cacheEnabled, sizePrefixDisabled, maxFrameSizeEnabled)
    0x01, // true
    0x01, // true
    0x01, // true
    0x01, // true
    0x01, // true
    0x00, // false
    // long: maxInactivityDuration = 0
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // long: maxInactivityDurationInitialDelay = 0
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // int: cacheSize = 0
    0x00, 0x00, 0x00, 0x00, // int: providerVersion = 0
    0x00, 0x00, 0x00, 0x00, // int: magic = 0
    0x00, 0x00, 0x00, 0x00,
];

// ###############################################
// OpenWire
pub struct OpenWire<'a> {
    address: &'a str,
    mq_name: &'a str,
    stream: Option<Box<TcpStream>>,
}

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

        self.wire_format_info_actions(&mut stream).await?;

        loop {
            if !self.check_connection(&mut stream).await? {
                break;
            }

            #[cfg(debug_assertions)]
            logger::info("main loop is reading!");

            sleep(Duration::from_secs(2)).await;
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
            .read_from_openwire(
                stream,
                OpenWireCommandTypes::ConnectionInfo,
                [0x00, 0x00, 0x00, 0x01],
            )
            .await?;

        let res = socket_content.starts_with(ACTIVEMQ_HEADER);

        #[cfg(debug_assertions)]
        logger::info(&format!(
            "The socket_content is starting with content  : {res}"
        ));

        Ok(true)
    }

    async fn wire_format_info_actions(&mut self, stream: &mut Box<TcpStream>) -> MQResult<()> {
        self.write_in_socket(stream, &WIRE_FORMAT_INFO).await?;

        let msg = self.read_from_socket(stream).await?;

        let mgs_str = String::from_utf8(msg)
            .map_err(|e| {
                MqError::TextErr(format!(
                    "Error in converting the response packet to string, {}",
                    e
                ))
            })?
            .trim()
            .to_string();

        #[cfg(debug_assertions)]
        logger::info(&format!("Openwire format message is '{}'", mgs_str));

        Ok(())
    }

    async fn read_from_openwire(
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

        self.write_in_socket(stream, &openwire_packet).await?;

        #[cfg(debug_assertions)]
        logger::info("The message has been written! going to read it");

        let packet_response_vec = self.read_from_socket(stream).await?;

        if !packet_response_vec.is_empty() {
            let recieved_message = String::from_utf8(packet_response_vec)
                .map_err(|e| {
                    MqError::TextErr(format!(
                        "Error in converting the response packet to string, {}",
                        e
                    ))
                })?
                .trim()
                .to_string();

            #[cfg(debug_assertions)]
            logger::info(&format!("The message is '{}'", recieved_message));

            // #[cfg(debug_assertions)]
            // logger::info(&format!("The message size: {}", recieved_message.len()));

            return Ok(recieved_message);
        }

        #[cfg(debug_assertions)]
        logger::info(&format!("The message is empty"));
        Ok("".into())
    }

    async fn write_in_socket(&mut self, stream: &mut Box<TcpStream>, bytes: &[u8]) -> MQResult<()> {
        if let Err(e) = stream.write(&bytes).await {
            logger::error(&format!(
                "Failed to write a message in the openwire socket! '{}'",
                e
            ));
            return Err(MqError::TextErr(
                "Failed to check the openwire connection!".into(),
            ));
        }
        Ok(())
    }

    async fn read_from_socket(&mut self, stream: &mut Box<TcpStream>) -> MQResult<Vec<u8>> {
        let mut buf = [0u8; SOCKET_BUFFER_SIZE];
        let mut packet_response_vec = Vec::new();

        loop {
            let read_size: usize = match stream.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    logger::error(&format!("Read error {}", e));
                    return Err(MqError::TextErr("Failed to read from socket!".into()));
                }
            };

            // #[cfg(debug_assertions)]
            // logger::info("In the reading loop!");

            packet_response_vec.extend_from_slice(&buf[..read_size]);

            if read_size < SOCKET_BUFFER_SIZE {
                break;
            }
        }

        Ok(packet_response_vec)
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
