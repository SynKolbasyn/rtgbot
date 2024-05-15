use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebAppData {
    data: String,
    button_text: String,
}
