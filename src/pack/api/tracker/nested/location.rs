use std::iter;

use serde::{Deserialize, Serialize};

use crate::pack::api::tracker::nested::{MapLocation, Section};
use crate::pack::rule::Rule;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub name: String,
    #[serde(default)]
    pub sections: Vec<Section>,
    #[serde(default)]
    pub access_rules: Vec<Rule>,
    #[serde(default)]
    pub map_locations: Vec<MapLocation>,
    #[serde(default)]
    pub children: Vec<Location>,
}

impl Location {
    pub fn child_locations_recursive(&self) -> Box<dyn Iterator<Item = &Location> + '_> {
        Box::new(
            self.children.iter().flat_map(|location| {
                iter::once(location).chain(location.child_locations_recursive())
            }),
        )
    }
}
