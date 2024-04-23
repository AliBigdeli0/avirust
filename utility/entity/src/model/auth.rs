pub trait Authentication<'a>: Sized {
    fn is_valid_credential(&self, username: &str, password: &str) -> bool;
}

pub struct StaticAuthentication;

impl StaticAuthentication {
    pub fn new() -> Self {
        StaticAuthentication {}
    }
}

impl Authentication<'_> for StaticAuthentication {
    fn is_valid_credential(&self, username: &str, password: &str) -> bool {
        username == "admin" && password.trim() == "admin"
    }
}
