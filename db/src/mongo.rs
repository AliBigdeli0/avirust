use mongodb:: {Client, error::Error, options::ClientOptions, Database, bson::doc};

use config::create_db_config_if_not_exist;

pub async fn init() {
    create_db_config_if_not_exist();
    
    let config = config::read_db_config().unwrap(); 

    check_db_exist(config).await.expect("error in db connection check! please check the database!");
}


async fn create_db(config: entity::model::db_config::DBConfig) -> Result<Box<(Client, Database)>, Error> {
    let client_options = Box::new(ClientOptions::parse(config.connection_string).await?);

    let timeout = client_options.connect_timeout.unwrap();
    let server_selection_timeout = client_options.server_selection_timeout.unwrap();
    let heart_beat = client_options.heartbeat_freq.unwrap();
    
    let client = Client::with_options(*client_options)?;

    logger::info(&format!("connection timeout: {:?}, server selection timeout: {:?}, heart beat : {:?}", 
        timeout, 
        server_selection_timeout, 
        heart_beat
    ));

    let database = client.database(&config.db_name);
    Ok(Box::new((client,database)))
}

pub async fn check_db_exist(config: entity::model::db_config::DBConfig) -> Result<(), Error> {
    let (client,database) = *(create_db(config).await?);

    let name = database.name();
    logger::info(&format!("the database name is: {}", name));


    let ping_doc = doc!{
        "ping": 1
    };
    client.database("admin").run_command(ping_doc, None).await.expect("error in database connection");

    logger::info("database connection has tested successfully!");
    Ok(())  
}