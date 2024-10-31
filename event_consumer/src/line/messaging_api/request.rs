pub mod source;
pub mod event;
pub mod emoji;
pub mod message;

use serde::{Deserialize, Serialize};
use crate::line::messaging_api::request::message::MessageObject;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventObjectRequest {
    pub destination: String,
    pub events: Vec<event::Event>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyMessageRequest {
    pub reply_token: String,
    pub messages: Vec<MessageObject>,
    pub notification_disabled: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::EventObjectRequest;
    use serde_json::from_str;

    #[test]
    fn test_message_event_deserialize() {
        // arrange
        let json_str = r#"
        {
          "destination": "xxxxxxxxxx",
          "events": [
            {
              "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
              "type": "message",
              "mode": "active",
              "timestamp": 1462629479859,
              "source": {
                "type": "group",
                "groupId": "Ca56f94637c...",
                "userId": "U4af4980629..."
              },
              "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
              "deliveryContext": {
                "isRedelivery": false
              },
              "message": {
                "id": "444573844083572737",
                "type": "text",
                "quoteToken": "q3Plxr4AgKd...",
                "text": "@All @example Good Morning!! (love)",
                "emojis": [
                  {
                    "index": 29,
                    "length": 6,
                    "productId": "5ac1bfd5040ab15980c9b435",
                    "emojiId": "001"
                  }
                ],
                "mention": {
                  "mentionees": [
                    {
                      "index": 0,
                      "length": 4,
                      "type": "all"
                    },
                    {
                      "index": 5,
                      "length": 8,
                      "userId": "U49585cd0d5...",
                      "type": "user"
                    }
                  ]
                }
              }
            }
          ]
        }
        "#;

        // act
        let request: serde_json::Result<EventObjectRequest> = from_str(json_str);

        println!("{:?}", request);
    }
}