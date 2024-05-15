use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    id: i64,
    #[serde(rename = "type")]
    type_: String,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    is_forum: Option<bool>,
}


impl Chat {
    
}
