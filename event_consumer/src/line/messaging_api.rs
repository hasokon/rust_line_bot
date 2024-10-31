pub mod request;

use std::time::Duration;
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{ClientBuilder, Url};
use crate::line::messaging_api::request::message::{MessageObject, Text};
use crate::line::messaging_api::request::ReplyMessageRequest;

pub async fn reply_simple_message(reply_token: &String, text: &String) -> Result<(), lambda_runtime::Error> {
    let request_data = ReplyMessageRequest {
        reply_token: reply_token.clone(),
        messages: vec![
            MessageObject::Text(Text {
                text: text.clone(),
                emojis: None,
                quote_token: None
            })
        ],
        notification_disabled: None,
    };
    // todo ハンドラ外で作成したい
    let client_builder = ClientBuilder::new();
    let client = client_builder.timeout(Duration::from_secs(1)).build()?;

    let url = Url::parse("https://api.line.me/v2/bot/message/reply")?;
    let token = std::env::var("LINE_CHANNEL_ACCESS_TOKEN")?;
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(format!("Bearer {}", token).as_str())?);

    let request_builder = client.post(url);
    let response = request_builder
        .headers(headers)
        .body(serde_json::to_string(&request_data)?)
        .send()
        .await?;

    debug!("LINE API Response: {:?}", response.text().await?);

    Ok(())
}