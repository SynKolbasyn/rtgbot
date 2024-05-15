use serde::{Deserialize, Serialize};

use crate::types::user::User;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoChatParticipantsInvited {
    users: Vec<User>,
}
