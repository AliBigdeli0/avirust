use rocket::{
    get,
    response::content::RawJson,
    response::status
};
use rocket::http::Status;

#[get("/is_ready")]
pub fn is_ready() -> status::Custom<RawJson<&'static str>> {
    status::Custom(Status::ImATeapot,RawJson("{'hi': 'world'}"))
}
