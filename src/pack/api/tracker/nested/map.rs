use serde::{Deserialize, Serialize};

use crate::util::value_or_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map {
    pub name: String,
    #[serde(deserialize_with = "value_or_string")]
    pub location_size: u32,
    #[serde(deserialize_with = "value_or_string")]
    pub location_border_thickness: u32,
    #[serde(default)]
    pub location_shape: LocationShape,
    pub img: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LocationShape {
    #[default]
    Rect,
    Diamond,
}
