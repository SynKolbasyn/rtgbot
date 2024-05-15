use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatBoostAdded {
    boost_count: i64,
}
