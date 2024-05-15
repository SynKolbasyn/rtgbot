use serde::{Deserialize, Serialize};

use crate::types::{
    background_fill::BackgroundFill,
    document::Document
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BackgroundType {
    BackgroundTypeFill(BackgroundTypeFill),
    BackgroundTypeWallpaper(BackgroundTypeWallpaper),
    BackgroundTypePattern(BackgroundTypePattern),
    BackgroundTypeChatTheme(BackgroundTypeChatTheme),
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundTypeFill {
    #[serde(rename = "type")]
    type_: String,
    fill: BackgroundFill,
    dark_theme_dimming: i64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundTypeWallpaper {
    #[serde(rename = "type")]
    type_: String,
    document: Document,
    dark_theme_dimming: i64,
    is_blurred: Option<bool>,
    is_moving: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundTypePattern {
    #[serde(rename = "type")]
    type_: String,
    document: Document,
    fill: BackgroundFill,
    intensity: i64,
    is_inverted: Option<bool>,
    is_moving: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundTypeChatTheme {
    #[serde(rename = "type")]
    type_: String,
    theme_name: String,
}
