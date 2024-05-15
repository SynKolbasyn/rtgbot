use serde::{Deserialize, Serialize};

use crate::types::photo_size::PhotoSize;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatShared {
    request_id: i64,
    chat_id: i64,
    title: Option<String>,
    username: Option<String>,
    photo: Option<Vec<PhotoSize>>,
}
