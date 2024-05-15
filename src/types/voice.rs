use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voice {
    file_id: String,
    file_unique_id: String,
    duration: i64,
    mime_type: Option<String>,
    file_size: Option<i64>,
}
