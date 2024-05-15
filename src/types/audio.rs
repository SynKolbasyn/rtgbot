use serde::{Deserialize, Serialize};

use crate::types::photo_size::PhotoSize;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Audio {
    file_id: String,
    file_unique_id: String,
    duration: i64,
    performer: Option<String>,
    title: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
    thumbnail: Option<PhotoSize>,
}
