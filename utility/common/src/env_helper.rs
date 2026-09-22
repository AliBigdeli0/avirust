use std::env;

pub struct EnvHelper {}

impl EnvHelper {
    pub fn get_redis_address() -> String {
        env::var("MED_REDIS_URL")
            .expect("MED_REDIS_URL env is not set!")
            .to_string()
    }

    pub fn get_amq_address() -> String {
        env::var("MED_AMQ_ADDRESS")
            .expect("MED_AMQ_ADDRESS env is not set!")
            .to_string()
    }

    pub fn get_mongo_address() -> String {
        env::var("MED_MONGO_CONN_STR")
            .expect("MED_MONGO_CONN_STR env is not set!")
            .to_string()
    }

    pub fn get_well_id() -> String {
        env::var("MED_WELL_ID")
            .expect("MED_WELL_ID env is not set!")
            .to_string()
    }

    pub fn get_stomp_username() -> String {
        env::var("MED_STOMP_USERNAME")
            .unwrap_or_else(|_| "admin".into())
            .to_string()
    }

    pub fn get_stomp_password() -> String {
        env::var("MED_STOMP_PASSWORD")
            .unwrap_or_else(|_| "admin".into())
            .to_string()
    }
}
