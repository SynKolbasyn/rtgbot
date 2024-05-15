use serde::{Deserialize, Serialize};

use crate::types::message::Message;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiveawayCompleted {
    winner_count: i64,
    unclaimed_prize_count: Option<i64>,
    // giveaway_message: Option<Message>,
}
