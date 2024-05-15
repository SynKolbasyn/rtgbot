use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoChatScheduled {
    start_date: i64,
}
