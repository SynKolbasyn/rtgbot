use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    user_id: Option<i64>,
    vcard: Option<String>,
}
