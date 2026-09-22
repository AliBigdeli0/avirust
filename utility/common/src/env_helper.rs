use std::env;

pub struct EnvHelper {}

impl EnvHelper {
    pub fn get_redis_address() -> String {
        env::var("MED_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1".to_string())
    }

    pub fn get_redis_port() -> u16 {
        env::var("MED_REDIS_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(6379)
    }

    pub fn get_redis_full_address() -> String {
        format!(
            "{}:{}",
            EnvHelper::get_redis_address(),
            EnvHelper::get_redis_port()
        )
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
