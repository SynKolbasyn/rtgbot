use serde::{Deserialize, Serialize};

use crate::types::{
    web_app_info::WebAppInfo,
    login_url::LoginUrl,
    switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat,
    callback_game::CallbackGame,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    callback_data: Option<String>,
    web_app: Option<WebAppInfo>,
    login_url: Option<LoginUrl>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
    switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    callback_game: Option<CallbackGame>,
    pay: Option<bool>,
}
