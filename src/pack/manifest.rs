use std::{fs, path::Path};

use eyre::{eyre, Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::pack::VariantUID;
use crate::BOM;

pub const FILENAME: &str = "manifest.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manifest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub game_name: String,
    pub package_uid: String,
    #[serde(default)]
    pub package_version: String,
    #[serde(default)]
    pub platform: String,
    #[serde(default)]
    pub platform_override: Option<String>,
    #[serde(default)]
    pub variants: IndexMap<VariantUID, Variant>,
}

impl Manifest {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let data = fs::read_to_string(path)
            .with_context(|| eyre!("failed to read manifest: {}", path.display()))?;
        let data = data.strip_prefix(BOM).unwrap_or(&data);
        let manifest = serde_hjson::from_str::<Manifest>(data)
            .with_context(|| eyre!("failed to parse manifest: {}", path.display()))?;

        Ok(manifest)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Variant {
    pub display_name: String,
    #[serde(default)]
    pub flags: Vec<String>,
}
