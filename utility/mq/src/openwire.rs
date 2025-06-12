use tokio::net::TcpStream;

use crate::mqerror::{self};

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
        let stream = TcpStream::connect(self.address)
            .await
            .map_err(|e| mqerror::MqError::TextErr(e.to_string()))?;

        self.stream = Some(Box::new(stream));

        Ok(())
    }

    pub async fn close(&mut self) {
        let _ = std::mem::take(&mut self.stream);
    }
}
