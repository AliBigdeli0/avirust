use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DBConfig {
    pub connection_string: String,
    pub db_name: String,
}

const AVI_DEFAULT_NAME: &str = "AVIDb";

impl DBConfig {
    pub fn init(connection_str: String) -> Self {
        Self {
            connection_string: connection_str,
            db_name: AVI_DEFAULT_NAME.into(),
        }
    }
}

impl Default for DBConfig {
    fn default() -> Self {
        Self {
            connection_string: String::from("mongodb://localhost:27017/?connectTimeoutMS=5000&serverSelectionTimeoutMS=5000&heartbeatFrequencyMS=5000").into(), 
            db_name: AVI_DEFAULT_NAME.into(),
        }
    }
}
