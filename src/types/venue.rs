use serde::{Deserialize, Serialize};

use crate::types::location::Location;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}
