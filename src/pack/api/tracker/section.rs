use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    pub name: Option<String>,
    // access_rules: TODO,
    // todo: more fields
}
