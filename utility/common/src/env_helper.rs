use std::env;

pub struct EnvHelper {}

impl EnvHelper {
    pub fn get_redis_address() -> String {
        env::var("MED_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string())
    }

    pub fn get_amq_address() -> String {
        env::var("MED_AMQ_ADDRESS")
            .unwrap_or_else(|_| "tcp://localhost:61616".to_string())
            .to_string()
    }

    pub fn get_well_id() -> String {
        env::var("MED_WELL_ID")
            .expect("MED_WELL_ID env is not set!")
            .to_string()
    }
}
