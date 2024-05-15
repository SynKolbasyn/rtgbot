use serde::{Deserialize, Serialize};

use crate::types::photo_size::PhotoSize;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharedUser {
    user_id: i64,
    first_name: Option<String>,
    last_name: Option<String>,
    username: Option<String>,
    photo: Option<Vec<PhotoSize>>,
}
