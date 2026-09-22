use mongodb::{Client, Database};
use std::sync::Arc;

use db::{dberror, mongo::read_all_wells};
use entity::model::db::{SubTypeValues, Well};

use crate::app_errors::{AppError, AppResult};

pub async fn read_well(mongo_address: &Arc<String>, well_id: &Arc<String>) -> AppResult<Arc<Well>> {
    let (client, database) = match init_db(&mongo_address).await {
        Ok((client, db)) => (client, db),
        Err(err) => {
            logger::error(&format!("{err}"));

            return Err(err.into());
        }
    };

    let well_data = match fetch_well(database, &well_id).await {
        Ok(well) => well,
        Err(err) => {
            logger::error(&format!("{err}"));

            return Err(err.into());
        }
    };

    let subtype_value = well_data.get_sub_type();
    if subtype_value != (SubTypeValues::AMQTextConsumer as i32) {
        let error_msg = format!(
            "DANGER ==> This program is not supporing any subtype except for AMQTextConsumer(32), current subtype is '{subtype_value}'"
        );
        logger::error(error_msg.as_str());
        return Err(AppError::Text(error_msg));
    }

    logger::info(&format!(
        "Well #{} is fetched from db (topic: '{}', mq name: '{}')",
        well_data.id,
        well_data.get_topic_name(),
        well_data.get_server_mq_name()
    ));

    client.shutdown().await;

    Ok(well_data)
}

async fn init_db(connection_str: &Arc<String>) -> dberror::Result<(Client, Database)> {
    logger::info("preparing database...");
    let (client, database) = db::mongo::init_with_connection_string(connection_str).await?;

    Ok((client, database))
}

async fn fetch_well(database: Database, well_id: &Arc<String>) -> dberror::Result<Arc<Well>> {
    let well = read_all_wells(database)
        .await?
        .into_iter()
        .find(|well| well.id == well_id.to_string());

    if let Some(well) = well {
        return Ok(Arc::new(well.clone()));
    } else {
        return Err(dberror::DbError::TextError(
            "Error in getting well from db".into(),
        ));
    }
}
