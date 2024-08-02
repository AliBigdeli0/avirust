use entity::{dto::excel_dto::ExcelInfo, model::db::*, serializer_helper::serialize_bson};
use mongodb::{bson::doc, error::Error, options::ClientOptions, Client, Cursor, Database};

use config::create_db_config_if_not_exist;

use crate::dberror;

pub async fn init() -> (Client, Database) {
    create_db_config_if_not_exist();

    let config = config::read_db_config().unwrap();

    check_db_exist(config)
        .await
        .expect("error in db connection check! please check the database!")
}

pub async fn init_with_connection_string(
    connection_string: &str,
) -> dberror::Result<(Client, Database)> {
    let config = entity::model::db_config::DBConfig::init(connection_string.into());

    check_db_exist(config)
        .await
        .map_err(|_| dberror::DbError::TextError("database does not running".into()))
}

async fn create_db(
    config: entity::model::db_config::DBConfig,
) -> dberror::Result<Box<(Client, Database)>> {
    let client_options: Box<ClientOptions> = Box::new(
        ClientOptions::parse(config.connection_string)
            .await
            .map_err(|_| dberror::DbError::TextError("error in parse configuration".into()))?,
    );

    let timeout = client_options
        .connect_timeout
        .ok_or(dberror::DbError::TextError(
            "error in convert timeout".into(),
        ))?;
    let server_selection_timeout =
        client_options
            .server_selection_timeout
            .ok_or(dberror::DbError::TextError(
                "error in convert server selection timeout".into(),
            ))?;

    let heart_beat = client_options
        .heartbeat_freq
        .ok_or(dberror::DbError::TextError(
            "error in convert heartbeat...".into(),
        ))?;

    let client = Client::with_options(*client_options)
        .map_err(|_| dberror::DbError::TextError("error in convert client with options".into()))?;

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
    let (client, database) = *(create_db(config)
        .await
        .map_err(|_| Error::custom("error in get data from create db"))?);

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

pub async fn read_all_wells<'a>(database: Database) -> dberror::Result<&'a Vec<Well>> {
    read_all_from_db::<Well>(database, WELL_COLLECTION_NAME)
        .await
        .map_err(|_| dberror::DbError::TextError("error in read all wells".into()))
}

pub async fn insert_time_from_excel(
    database: &Database,
    excel_data: ExcelInfo,
) -> Result<(), Error> {
    let hash = &excel_data.to_hash();

    if hash.len() == 0 {
        return Err(Error::custom(
            "the hash len is 0, it can not converted to a valid data",
        ));
    }
    let time_data = &TimeData::new_hash(excel_data.date, hash);

    let column_name = TimeData::generate_collection_name(excel_data.get_well_id(), excel_data.date);

    let bson_data = serialize_bson(&time_data)?;
    database
        .collection(&column_name)
        .insert_one(bson_data, None)
        .await?;

    Ok(())
}

pub async fn remove_time_from_excel(
    database: &Database,
    excel_data: ExcelInfo,
) -> Result<(), Error> {
    let time_data = &TimeData::new(excel_data.date, "not_important".to_string());

    let column_name = TimeData::generate_collection_name(excel_data.get_well_id(), excel_data.date);

    let filter = doc! {
        "d": time_data.date
    };

    database
        .collection::<TimeData>(&column_name)
        .delete_one(filter, None)
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
    T: serde::de::DeserializeOwned + 'a,
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
