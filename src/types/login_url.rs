use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUrl {
    url: String,
    forward_text: Option<String>,
    bot_username: Option<String>,
    request_write_access: Option<bool>,
}
