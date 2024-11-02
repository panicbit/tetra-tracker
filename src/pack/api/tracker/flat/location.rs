use crate::pack::api::tracker::flat::Section;
use crate::pack::api::tracker::{nested, AsId, Id};
use crate::pack::rule::Rule;

#[derive(Debug, Clone)]
pub struct Location {
    pub id: Id<Location>,
    pub parent: Option<Id<Location>>,
    pub children: Vec<Id<Location>>,
    pub sections: Vec<Section>,
    pub name: String,
    pub access_rules: Vec<Rule>,
    pub map_locations: Vec<nested::MapLocation>,
}

impl AsId<Location> for Location {
    fn as_id(&self) -> Id<Location> {
        self.id.clone()
    }
}
