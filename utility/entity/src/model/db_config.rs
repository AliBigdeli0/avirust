use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DBConfig {
    pub connection_string: String,
    pub db_name: String,
}

impl Default for DBConfig {
    fn default() -> Self {
        Self {
            connection_string: String::from("mongodb://localhost:27017/?connectTimeoutMS=5000&serverSelectionTimeoutMS=5000&heartbeatFrequencyMS=5000"), 
            db_name: String::from("AVIDb"),
        }
    }
}
