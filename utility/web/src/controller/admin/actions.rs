use entity::dto::default_dto::IsReady;
use rocket::{
    get,
    serde::json::Json
};


#[get("/is_ready")]
pub async fn is_ready() -> Json<IsReady<'static>> {
    let mut is_ready: IsReady<'static>= IsReady::default();
    is_ready.message = &"admin is ready";
    Json(is_ready)
}
