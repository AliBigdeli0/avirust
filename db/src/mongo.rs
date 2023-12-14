use entity::{dto::excel_dto::ExcelInfo, model::db::*, serializer_helper::serialize_bson};
use mongodb::{bson::doc, error::Error, options::ClientOptions, Client, Cursor, Database};

use config::create_db_config_if_not_exist;

pub async fn init() -> (Client, Database) {
    create_db_config_if_not_exist();

    let config = config::read_db_config().unwrap();

    check_db_exist(config)
        .await
        .expect("error in db connection check! please check the database!")
}

async fn create_db(
    config: entity::model::db_config::DBConfig,
) -> Result<Box<(Client, Database)>, Error> {
    let client_options = Box::new(ClientOptions::parse(config.connection_string).await?);

    let timeout = client_options.connect_timeout.unwrap();
    let server_selection_timeout = client_options.server_selection_timeout.unwrap();
    let heart_beat = client_options.heartbeat_freq.unwrap();

    let client = Client::with_options(*client_options)?;

    logger::info(&format!(
        "connection timeout: {:?}, server selection timeout: {:?}, heart beat : {:?}",
        timeout, server_selection_timeout, heart_beat
    ));

    let database = client.database(&config.db_name);
    Ok(Box::new((client, database)))
}

pub async fn check_db_exist(
    config: entity::model::db_config::DBConfig,
) -> Result<(Client, Database), Error> {
    let (client, database) = *(create_db(config).await?);

    let name = database.name();
    logger::info(&format!("the database name is: {}", name));

    let ping_doc = doc! {
        "ping": 1
    };
    
    client
        .database("admin")
        .run_command(ping_doc, None)
        .await
        .expect("error in database connection");

    logger::info("database connection has tested successfully!");
    Ok((client, database))
}

pub async fn read_all_wells<'a>(database: Database) -> Result<&'a Vec<Well>, Error> {
    read_all_from_db::<Well>(database, WELL_COLLECTION_NAME).await
}

pub async fn insert_time_from_excel(
    database: &Database,
    excel_data: ExcelInfo,
) -> Result<(), Error> {
    let hash = &excel_data.to_hash();
    let time_data = &TimeData::new_hash(excel_data.date, hash);

    let column_name = TimeData::generate_collection_name(excel_data.get_well_id(), excel_data.date);

    let bson_data = serialize_bson(&time_data)?;
    database
        .collection(&column_name)
        .insert_one(bson_data, None)
        .await?;

    Ok(())
}

pub async fn read_from_time_data<'a>(
    database: Database,
    collection_name: &'a str,
) -> Result<&'a Vec<TimeData>, Error> {
    read_all_from_db::<TimeData>(database, collection_name).await
}

async fn read_all_from_db<'a, T>(
    database: Database,
    collection_name: &'a str,
) -> Result<&'a Vec<T>, Error>
where
    T: serde::de::DeserializeOwned,
{
    let mut return_documents: Vec<T> = Vec::new();

    let mut cursor: Cursor<T> = database
        .collection(collection_name)
        .find(None, None)
        .await?;

    while cursor.advance().await? {
        let well = cursor.deserialize_current()?;
        return_documents.push(well);
    }

    let boxed_vec: Box<Vec<T>> = Box::new(return_documents);
    let vec_ref: &'a Vec<T> = Box::leak(boxed_vec);
    Ok(vec_ref)
}
