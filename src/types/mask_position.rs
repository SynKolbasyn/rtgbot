use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaskPosition {
    point: String,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}
