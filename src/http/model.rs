use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub id: String,
    pub long_url: String,
}
