use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DBConfig {
    pub connection_string: String,
}

impl Default for DBConfig {
    fn default() -> Self {
        Self {
            connection_string: String::from("127.0.0.1:27017")
        }
    }
}