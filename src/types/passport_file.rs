use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassportFile {
    file_id: String,
    file_unique_id: String,
    file_size: i64,
    file_date: i64,
}
