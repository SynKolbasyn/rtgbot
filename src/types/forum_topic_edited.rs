use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumTopicEdited {
    name: Option<String>,
    icon_custom_emoji_id: Option<String>,
}
