use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumTopicCreated {
    name: String,
    icon_color: i64,
    icon_custom_emoji_id: Option<String>,
}
