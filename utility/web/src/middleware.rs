use rocket::fairing::{Fairing, Info, Kind};

#[derive(Default)]
pub struct Credential {
    _username: String,
    _password: String,
}

impl<'a> Fairing for Credential {
    fn info(&self) -> Info {
        Info {
            name: "creadential",
            kind: Kind::Request | Kind::Response,
        }
    }
}
