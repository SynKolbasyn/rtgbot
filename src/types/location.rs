use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<i64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
}
