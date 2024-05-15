use serde::{Deserialize, Serialize};

use crate::types::background_type::BackgroundType;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatBackground {
    #[serde(rename = "type")]
    type_: BackgroundType,
}
