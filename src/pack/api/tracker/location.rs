use serde::{Deserialize, Serialize};

use crate::pack::api::tracker::{MapLocation, Section};
use crate::util::value_or_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub name: String,
    #[serde(default)]
    pub sections: Vec<Section>,
    // access_rules: TODO,
    #[serde(default)]
    pub map_locations: Vec<MapLocation>,
}
