use std::{fs, path::Path};

use eyre::{Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::BOM;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manifest {
    pub name: String,
    pub author: String,
    pub game_name: String,
    pub package_uid: String,
    pub package_version: String,
    pub platform: String,
    pub platform_override: Option<String>,
    pub variants: IndexMap<String, Variant>,
}

impl Manifest {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let data = fs::read_to_string(path).context("failed to read manifest")?;
        let data = data.strip_prefix(BOM).unwrap_or(&data);
        let manifest =
            serde_hjson::from_str::<Manifest>(data).context("failed to parse manifest")?;

        Ok(manifest)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Variant {
    display_name: String,
    #[serde(default)]
    flags: Vec<String>,
}
