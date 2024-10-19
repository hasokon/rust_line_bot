use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Source {
    #[serde(rename_all = "camelCase")]
    User {
        user_id: String,
    },
    #[serde(rename_all = "camelCase")]
    Group {
        group_id: String,
        user_id: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Room {
        room_id: String,
        user_id: Option<String>,
    }
}