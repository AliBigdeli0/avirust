use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DBConfig {
    pub connection_string: String,
    pub db_name: String,
}

impl Default for DBConfig {
    fn default() -> Self {
        Self {
            connection_string: String::from("mongodb://127.0.0.1:27017"), 
            db_name: String::from("AVIDb"),
        }
    }
}