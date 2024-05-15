use serde::{Deserialize, Serialize};

use crate::types::message_entity::MessageEntity;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextQuote {
    text: String,
    entities: Option<Vec<MessageEntity>>,
    position: i64,
    is_manual: Option<bool>,
}
