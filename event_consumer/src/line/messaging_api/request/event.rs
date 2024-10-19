mod message;

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};
use crate::line::messaging_api::request::event::message::MessageEvent;
use crate::line::messaging_api::request::source;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelMode {
    Active,
    Standby,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryContext {
    pub is_redelivery: bool,
}
fn from_unix_timestamp_ms<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;  // 13桁のUnixタイムを取得
    Utc.timestamp_millis_opt(timestamp)
        .single()
        .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventCommonField {
    pub mode: ChannelMode,
    #[serde(deserialize_with = "from_unix_timestamp_ms")]
    pub timestamp: DateTime<Utc>,
    pub source: Option<source::Source>,
    pub webhook_event_id: String,
    pub delivery_context: DeliveryContext,
}


#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Event {
    Message(MessageEvent)
}