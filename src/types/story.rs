use serde::{Deserialize, Serialize};

use crate::types::chat::Chat;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Story {
    chat: Chat,
    id: i64,
}
