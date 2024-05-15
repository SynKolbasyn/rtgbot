use serde::{Deserialize, Serialize};

use crate::types::{
    photo_size::PhotoSize,
    file::File,
    mask_position::MaskPosition,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sticker {
    file_id: String,
    file_unique_id: String,
    #[serde(rename = "type")]
    type_: String,
    width: i64,
    height: i64,
    is_animated: bool,
    is_video: bool,
    thumbnail: Option<PhotoSize>,
    emoji: Option<String>,
    set_name: Option<String>,
    premium_animation: Option<File>,
    mask_position: Option<MaskPosition>,
    custom_emoji_id: Option<String>,
    needs_repainting: Option<bool>,
    file_size: Option<i64>,
}
