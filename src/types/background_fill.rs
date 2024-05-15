use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BackgroundFill {
    BackgroundFillSolid(BackgroundFillSolid),
    BackgroundFillGradient(BackgroundFillGradient),
    BackgroundFillFreeformGradient(BackgroundFillFreeformGradient),
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundFillSolid {
    #[serde(rename = "type")]
    type_: String,
    color: i64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundFillGradient {
    #[serde(rename = "type")]
    type_: String,
    top_color: i64,
    bottom_color: i64,
    rotation_angle: i64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundFillFreeformGradient {
    #[serde(rename = "type")]
    type_: String,
    colors: Vec<i64>,
}
