use std::{fs, path::Path};

use eyre::{Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::BOM;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manifest {
    name: String,
    author: String,
    game_name: String,
    package_uid: String,
    package_version: String,
    platform: String,
    platform_override: Option<String>,
    variants: IndexMap<String, Variant>,
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
