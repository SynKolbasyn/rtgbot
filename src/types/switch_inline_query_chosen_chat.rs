use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwitchInlineQueryChosenChat {
    query: Option<String>,
    allow_user_chats: Option<bool>,
    allow_bot_chats: Option<bool>,
    allow_group_chats: Option<bool>,
    allow_channel_chats: Option<bool>,
}
