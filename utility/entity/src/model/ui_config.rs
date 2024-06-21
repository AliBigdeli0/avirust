use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    net::{SocketAddr, ToSocketAddrs},
    rc::Rc,
};
use thiserror::Error;

//errors
#[derive(Error, Debug)]
pub enum UiConfigError {
    #[error("error read config: {0}")]
    Msg(String),
}

//
#[derive(Serialize, Deserialize, Clone)]
pub struct UIConfig {
    server_address: String,
    verbose: bool,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            server_address: "localhost:8066".to_owned(),
            verbose: false,
        }
    }
}

impl UIConfig {
    pub fn change_address(&mut self, addr: String) {
        self.server_address = addr;
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }

    pub fn get_ip(&self) -> Result<String, UiConfigError> {
        let adr = self.to_address()?;
        Ok(adr.ip().to_string())
    }

    pub fn get_port(&self) -> Result<u16, UiConfigError> {
        let adr = self.to_address()?;
        Ok(adr.port())
    }

    fn to_address(&self) -> Result<SocketAddr, UiConfigError> {
        let mut adr = self
            .server_address
            .to_socket_addrs()
            .map_err(|e| UiConfigError::Msg(e.to_string()))?;
        if let Some(addr) = adr.next() {
            Ok(addr)
        } else {
            Err(UiConfigError::Msg("error in get message".to_owned()))
        }
    }
}

//
#[derive(Clone)]
pub struct UICommonMemory {
    pub config: Rc<RefCell<UIConfig>>,
}

impl UICommonMemory {
    pub fn new(conf: UIConfig) -> Self {
        Self {
            config: Rc::new(RefCell::new(conf)),
        }
    }

    pub fn change_address(&self, addr: String) {
        let mut config = self.config.borrow_mut();
        config.change_address(addr);
    }

    pub fn set_verbose(&self, verbose: bool) {
        let mut config = self.config.borrow_mut();
        config.set_verbose(verbose);
    }
}
// ------------------------------------------
pub trait Authenticator {
    fn is_valid(&self, username: String, password: String) -> bool;
}

// simple authenticator
pub struct SimpleAuthenticator;

impl Authenticator for SimpleAuthenticator {
    fn is_valid(&self, username: String, password: String) -> bool {
        username.eq("admin") && password.eq("admin")
    }
}

#[cfg(test)]
pub mod test_uiconfig {
    use super::*;

    #[test]
    fn test_address() {
        let mut config = UIConfig::default();
        let ip = config.get_ip().unwrap();
        let port = config.get_port().unwrap();
        assert_eq!("::1", ip);
        assert_eq!(8066, port);

        config.change_address("10.10.10.101:9004".to_owned());

        assert_eq!("10.10.10.101", config.get_ip().unwrap());
        assert_eq!(9004, config.get_port().unwrap());
    }
}
