use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedCredentials {
    data: String,
    hash: String,
    secret: String,
}
