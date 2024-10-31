use crate::line::messaging_api::request::event::EventCommonField;
use serde::Deserialize;
use crate::line::messaging_api::request::emoji::Emoji;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mentionee {
    pub index: u32,
    pub length: u32,
    #[serde(rename = "type")]
    pub mentionee_type: String,
    pub user_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub mentionees: Vec<Mentionee>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Message {
    #[serde(rename_all = "camelCase")]
    Text {
        id: String,
        quote_token: String,
        text: String,
        emojis: Option<Vec<Emoji>>,
        mention: Option<Mention>,
        quoted_message_id: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEvent {
    #[serde(flatten)]
    pub common: EventCommonField,
    pub reply_token: String,
    pub message: Message,
}