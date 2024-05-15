use serde::{Deserialize, Serialize};

use crate::types::shared_user::SharedUser;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsersShared {
    request_id: i64,
    users: Vec<SharedUser>,
}
