use serde::{Deserialize, Serialize};

use crate::util::value_or_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    pub name: Option<String>,
    // access_rules: TODO,
    // todo: more fields
}
