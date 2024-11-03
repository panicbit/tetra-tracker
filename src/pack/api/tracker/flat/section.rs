use crate::pack::api::tracker::flat::Location;
use crate::pack::api::tracker::{AsId, Id};
use crate::pack::rule::Rule;

#[derive(Debug, Clone)]
pub struct Section {
    pub id: Id<Section>,
    pub parent: Id<Location>,
    pub name: String,
    pub access_rules: Vec<Rule>,
    // todo: more fields
}

impl AsId<Section> for Section {
    fn as_id(&self) -> Id<Section> {
        self.id.clone()
    }
}
