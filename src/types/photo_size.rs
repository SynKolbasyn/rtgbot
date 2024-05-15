use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoSize {
    file_id: String,
    file_unique_id: String,
    width: i64,
    height: i64,
    file_size: Option<i64>,
}
