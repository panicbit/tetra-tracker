use serde::{Deserialize, Serialize};

use crate::util::value_or_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapLocation {
    pub map: String,
    #[serde(deserialize_with = "value_or_string")]
    pub x: i32,
    #[serde(deserialize_with = "value_or_string")]
    pub y: i32,
}
