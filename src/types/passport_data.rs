use serde::{Deserialize, Serialize};

use crate::types::{
    encrypted_passport_element::EncryptedPassportElement,
    encrypted_credentials::EncryptedCredentials,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassportData {
    data: Vec<EncryptedPassportElement>,
    credentials: EncryptedCredentials,
}
