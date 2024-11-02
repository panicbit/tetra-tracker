use serde::{Deserialize, Serialize};

use crate::pack::rule::Rule;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    pub name: Option<String>,
    #[serde(default)]
    pub access_rules: Vec<Rule>,
    // todo: more fields
}
