use serde::{Deserialize, Serialize};

use crate::types::passport_file::PassportFile;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    type_: String,
    data: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    files: Option<Vec<PassportFile>>,
    front_side: Option<PassportFile>,
    reverse_side: Option<PassportFile>,
    selfie: Option<PassportFile>,
    translation: Option<Vec<PassportFile>>,
    hash: String,
}
