use serde::{Deserialize, Serialize};

use crate::types::user::User;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    type_: String,
    offset: i64,
    length: i64,
    url: Option<String>,
    user: Option<User>,
    language: Option<String>,
    custom_emoji_id: Option<String>,
}
