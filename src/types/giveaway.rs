use serde::{Deserialize, Serialize};

use crate::types::chat::Chat;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Giveaway {
    chats: Vec<Chat>,
    winners_selection_date: i64,
    winner_count: i64,
    only_new_members: Option<bool>,
    has_public_winners: Option<bool>,
    prize_description: Option<String>,
    country_codes: Option<Vec<String>>,
    premium_subscription_month_count: Option<i64>,
}
