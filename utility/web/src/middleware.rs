use std::sync::Arc;

use base64::{self, engine::general_purpose, Engine};
use entity::model::auth::Authentication;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    Data, Request, Response,
};

const AUTHORIZATION_STR: &str = "Authorization";

#[derive(Default)]
pub struct Credential<T>
where
    T: Authentication<'static>,
{
    _auth: Arc<T>,
}

impl<'a, T> Credential<T>
where
    T: Authentication<'static>,
{
    pub fn new(auth: Arc<T>) -> Self {
        Credential { _auth: auth }
    }

    fn failed_request<'r>(&self, res: &mut Response<'r>) {
        let res_string = String::from("request failed!!!");
        res.set_status(Status::Unauthorized);

        let reopen_header = Header::new("WWW-Authenticate", "Basic realm=\"Restricted area\"");
        res.set_header(reopen_header);
        res.set_sized_body(res_string.len(), std::io::Cursor::new(res_string));
    }

    fn check_admin_permission<'r>(&self, req: &'r Request<'_>, _res: &mut Response<'r>) -> bool {
        let tolower_case_check = String::from(AUTHORIZATION_STR).to_lowercase();
        let auth_header = req.headers().get_one(&tolower_case_check);
        if let Some(parsed_auth_header) = auth_header {
            if parsed_auth_header.starts_with("Basic ") {
                let encoded_credential = parsed_auth_header.trim_start_matches("Basic ");
                if let Ok(decode_credential_vec) =
                    general_purpose::STANDARD.decode(encoded_credential)
                {
                    let decode_credential_str = String::from_utf8_lossy(&decode_credential_vec);
                    let splited_credential: Vec<&str> = decode_credential_str.split(":").collect();
                    let (username, password) = (splited_credential[0], splited_credential[1]);
                    return self._auth.is_valid_credential(username, password);
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }

        false
    }

    fn remove_authentication_header<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.remove_header(AUTHORIZATION_STR);
        res.remove_header(&AUTHORIZATION_STR.to_lowercase());

        self.failed_request(res);
    }
}

#[rocket::async_trait]
impl<'a, T> Fairing for Credential<T>
where
    T: Authentication<'static> + Send + Sync + 'static,
{
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Credential",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, _request: &mut Request<'_>, _: &mut Data<'_>) {}

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        if req.uri().path().contains("admin") {
            if !self.check_admin_permission(req, res) {
                self.failed_request(res)
            }
        } else if req.uri().path().contains("logout") {
            self.remove_authentication_header(req, res)
        }
    }
}
