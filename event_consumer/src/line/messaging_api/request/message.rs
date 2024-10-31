use serde::Serialize;
use crate::line::messaging_api::request::emoji::Emoji;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub text: String,
    pub emojis: Option<Vec<Emoji>>,
    pub quote_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MessageObject {
    Text(Text)
}