use serde::{Deserialize, Serialize};

use crate::types::photo_size::PhotoSize;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animation {
    file_id: String,
    file_unique_id: String,
    width: i64,
    height: i64,
    duration: i64,
    thumbnail: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}
