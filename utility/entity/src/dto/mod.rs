pub mod default_dto;
pub mod excel_dto;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
