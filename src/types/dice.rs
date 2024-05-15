use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dice {
    emoji: String,
    value: i64,
}
