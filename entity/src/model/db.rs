use base64::Engine;
use base64::engine::general_purpose;
use bson::{doc, oid::ObjectId, Binary, DateTime as BsonDateTime};
use chrono::{Utc, Datelike, DateTime as ChronoDateTime};
use chrono::prelude::*;
use serde::Deserialize;

pub const WELL_COLLECTION_NAME:&str = "wells";
pub const START_DATE: i32 = 2000;

pub fn convert_to_bson(d: ChronoDateTime<Utc>) -> BsonDateTime {
    bson::DateTime::builder()
        .year(d.year())
        .month(d.month()as u8)
        .day(d.day() as u8)
        .hour(d.hour() as u8)
        .minute(d.minute() as u8)
        .second(d.second() as u8)
        .build()
        .unwrap()
}

#[derive(Debug,Deserialize)]
#[allow(dead_code)]
pub struct Well {
    _id: ObjectId, 
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="ID")]
    pub id: String,
    #[serde(rename="IP")]
    pub ip: String,
    #[serde(rename="IsActive")]
    pub is_active: bool,
}

#[derive(Debug,Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct TimeData {
    _id:ObjectId, 
    
    #[serde(rename="l")]
    pub data: Binary,


    #[serde(rename="d")]
    pub date: BsonDateTime,
}

impl TimeData {

    pub fn new(d: ChronoDateTime<Utc>, data_str: String) -> Self {
        let data = general_purpose::STANDARD_NO_PAD.encode(data_str);
        let binary_data = Binary::from_base64(data,None).unwrap();
        let bson_date = convert_to_bson(d);

        Self {
            _id: ObjectId::new(),
            data:binary_data, 
            date: bson_date,
        }
    }

    pub fn generate_collection_name(id: &str, date: ChronoDateTime<Utc>) -> String {
        let year = date.year() - START_DATE;
        format!("z_z_{}_{:02}_{:02}_{:02}",id, year, date.month(), date.day())
    }
}

#[cfg(test)]
mod test_db {

    use super::*;

    #[test]
    fn test_generate_datetime_column() {
        let utc = common::time_helper::create_datetime_utc(2023, 10, 30, 12, 0, 0);
         
        let date_str = TimeData::generate_collection_name("tt", utc);
        assert_eq!(date_str, "z_z_tt_23_10_30");

        let utc = common::time_helper::create_datetime_utc(2023, 3, 4, 12, 0, 0);
         
        let date_str = TimeData::generate_collection_name("tt", utc);
        assert_eq!(date_str, "z_z_tt_23_03_04");
    }

    #[test]
    fn check_create_bson() {
        let time_data = TimeData::new(common::time_helper::get_now_utc(),String::from("data"));

        assert_ne!(time_data.data.bytes.len(), 0);
        assert_ne!(time_data.date.to_string(), "")
    }
}

