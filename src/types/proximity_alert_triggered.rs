use serde::{Deserialize, Serialize};

use crate::types::user::User;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProximityAlertTriggered {
    traveler: User,
    watcher: User,
    distance: i64,
}
