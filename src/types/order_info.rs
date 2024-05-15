use serde::{Deserialize, Serialize};

use crate::types::shipping_address::ShippingAddress;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}
