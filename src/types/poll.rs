use serde::{Deserialize, Serialize};

use crate::types::{
    message_entity::MessageEntity,
    poll_option::PollOption,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Poll {
    id: String,
    question: String,
    question_entities: Option<Vec<MessageEntity>>,
    options: Vec<PollOption>,
    total_voter_count: i64,
    is_closed: bool,
    is_anonymous: bool,
    #[serde(rename = "type")]
    type_: String,
    allows_multiple_answers: bool,
    correct_option_id: Option<i64>,
    explanation: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<i64>,
    close_date: Option<i64>,
}
