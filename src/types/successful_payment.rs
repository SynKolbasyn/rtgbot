use serde::{Deserialize, Serialize};

use crate::types::order_info::OrderInfo;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessfulPayment {
    currency: String,
    total_amount: i64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id: String,
}
