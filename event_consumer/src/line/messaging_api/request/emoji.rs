use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub index: u32,
    pub length: Option<u32>,
    pub product_id: String,
    pub emoji_id: String,
}