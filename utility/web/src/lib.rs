#![deny(warnings)]

mod controller;
mod middleware;

#[macro_use]
extern crate rocket;
extern crate config;
extern crate entity;
extern crate logger;

use entity::model::auth::Authentication;
use middleware::Credential;
use rocket::config::Config;
use rocket::fs::FileServer;
use std::net::IpAddr;

use std::str::FromStr;
use std::sync::Arc;

pub fn get_version() -> &'static str {
    "1.0.0"
}

pub fn init() {
    config::create_server_config_if_not_exist();
}

pub async fn run_wait<T>(authen: T)
where
    T: Authentication<'static> + Sync + Send + 'static,
{
    let server_config = config::read_server_config().unwrap();
    logger::info(&format!(
        "going to listen to: {}:{}",
        server_config.server.listen, server_config.server.port
    ));

    let mut config = Config::default();
    config.port = server_config.server.port as u16;
    config.address = IpAddr::from_str(&server_config.server.listen).unwrap();

    let credential = Credential::new(Arc::new(authen));

    if let Err(_err) = rocket::custom(config)
        .attach(credential)
        .mount("/api/v1", routes![controller::actions::is_ready])
        .mount(
            "/api/v1/admin",
            routes![
                controller::admin::actions::is_ready,
                controller::admin::actions::auth
            ],
        )
        .mount("/", FileServer::from("static/"))
        .launch()
        .await
    {
        logger::info("can't run the rocket!!!!");
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        assert_eq!(get_version(), "1.0.0")
    }

    #[test]
    fn convert_string_to_ipv4_address() {
        let address = String::from("127.0.0.1");
        let ip_address = IpAddr::from_str(&address).unwrap();
        assert!(ip_address.is_ipv4());
    }
}
