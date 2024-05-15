use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    file_id: String,
    file_unique_id: String,
    file_size: Option<i64>,
    file_path: Option<String>,
}
