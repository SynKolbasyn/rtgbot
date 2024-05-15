use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageAutoDeleteTimerChanged {
    message_auto_delete_time: i64,
}
