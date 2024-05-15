use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}
