use serde::{Deserialize, Serialize};

use crate::types::photo_size::PhotoSize;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoNote {
    file_id: String,
    file_unique_id: String,
    length: i64,
    duration: i64,
    thumbnail: Option<PhotoSize>,
    file_size: Option<i64>,
}
