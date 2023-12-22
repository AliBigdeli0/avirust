use entity::dto::default_dto::IsReady;
use rocket::{
    get,
    serde::json::Json
};

#[get("/is_ready")]
pub async fn is_ready() -> Json<IsReady<'static>> {
    let is_ready: IsReady<'static>= IsReady::default();
     
    Json(is_ready)
}
