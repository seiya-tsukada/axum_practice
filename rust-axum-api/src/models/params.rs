use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    pub id: Option<u8>,
}