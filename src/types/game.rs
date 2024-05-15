use serde::{Deserialize, Serialize};

use crate::types::{
    photo_size::PhotoSize,
    message_entity::MessageEntity,
    animation::Animation,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}
