extern crate config;
extern crate mongodb;

use mongodb:: {Client, error::Error};

use config::create_db_config_if_not_exist;

pub fn get_db_version() -> &'static str {
    "1.0.0"
}

pub async fn init() {
    create_db_config_if_not_exist();
    
    let config = config::read_db_config().unwrap(); 

    check_db_exist(config.connection_string, config.db_name).await.unwrap();
}

pub async fn check_db_exist(con_str: String, db_name: String) -> Result<(), Error> {
    let client = Client::with_uri_str(con_str).await?;
    let database = client.database(&db_name);
    let _ = database.name();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_version() {
        let version = get_db_version();
        assert_eq!(version, "1.0.0");
    }
}
