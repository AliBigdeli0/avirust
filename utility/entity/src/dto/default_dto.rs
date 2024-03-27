use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct IsReady<'a> {
    id: usize,
    pub message: &'a str,
}

impl Default for IsReady<'_> {
    fn default() -> Self {
        Self {
            id: 0,
            message: "ready",
        }
    }
}
