use serde::{Deserialize, Serialize};

use crate::types::inline_keyboard_button::InlineKeyboardButton;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
