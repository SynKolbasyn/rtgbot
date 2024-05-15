use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: i64,
}