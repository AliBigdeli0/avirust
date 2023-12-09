use bson::{doc, oid::ObjectId};
use serde::Deserialize;

pub const WELL_COLLECTION_NAME:&str = "wells";

#[derive(Debug,Deserialize)]
#[allow(non_snake_case,dead_code)]
pub struct Well {
    _id: ObjectId, 
    pub Name: String,
    pub ID: String,
    pub IP: String,
    pub IsActive: bool,
}




