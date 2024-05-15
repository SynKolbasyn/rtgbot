use serde::{Deserialize, Serialize};

use crate::types::message_entity::MessageEntity;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollOption {
    text: String,
    text_entities: Option<Vec<MessageEntity>>,
    voter_count: i64,
}
