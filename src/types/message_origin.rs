use serde::{Deserialize, Serialize};
use crate::types::{
    chat::Chat,
    user::User,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageOrigin {
    MessageOriginUser(MessageOriginUser),
    MessageOriginHiddenUser(MessageOriginHiddenUser),
    MessageOriginChat(MessageOriginChat),
    MessageOriginChannel(MessageOriginChannel),
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageOriginUser {
    #[serde(rename = "type")]
    type_: String,
    date: i64,
    sender_user: User,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageOriginHiddenUser {
    #[serde(rename = "type")]
    type_: String,
    date: i64,
    sender_user_name: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageOriginChat {
    #[serde(rename = "type")]
    type_: String,
    date: i64,
    sender_chat: Chat,
    author_signature: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageOriginChannel {
    #[serde(rename = "type")]
    type_: String,
    date: i64,
    chat: Chat,
    message_id: i64,
    author_signature: Option<String>,
}
