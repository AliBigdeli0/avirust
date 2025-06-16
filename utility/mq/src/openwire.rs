use tokio::net::TcpStream;
use url::Url;

use crate::mqerror::{self, MqError};

pub struct OpenWire<'a> {
    address: &'a str,
    stream: Option<Box<TcpStream>>,
}

impl<'a> OpenWire<'a> {
    pub fn new(address: &'a str) -> Self {
        Self {
            address,
            stream: None,
        }
    }

    pub async fn start(&mut self) -> mqerror::Result<()> {
        let (address_str, port) = self.parse_url()?;
        let stream = TcpStream::connect(format!("{address_str}:{port}"))
            .await
            .map_err(|e| mqerror::MqError::TextErr(e.to_string()))?;

        self.stream = Some(Box::new(stream));

        logger::info("Openwire tcp connection has been created!");
        Ok(())
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
