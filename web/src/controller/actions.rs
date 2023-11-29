use rocket::get;

#[get("/is_ready")]
fn index() -> String {
    String::from("ok")
}