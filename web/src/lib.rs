mod controller;

extern crate rocket;

pub fn get_version() -> &'static str {
    "1.0.0"
}

pub fn init() {
    config::create_server_config_if_not_exist();
}

pub fn run_wait() {
    let server_config= config::read_server_config().unwrap();
    logger::info(&format!("going to listen to: {}:{}", server_config.server.listen, server_config.server.port));

    rocket::build()
        .mount(
            "/api/v1",
            openapi_get_routes![

            ]
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        assert_eq!(get_version(), "1.0.0")
    }
}

