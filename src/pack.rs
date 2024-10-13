use std::path::{Path, PathBuf};
use std::sync::Arc;

use eyre::{eyre, Context, Result};
pub use manifest::Manifest;
use mlua::Lua;
use serde::{Deserialize, Serialize};

use crate::pack::api::Api;

pub mod api;
pub mod manifest;

pub struct Pack {
    pub root: PathBuf,
    pub manifest: Manifest,
    pub api: Api,
}

impl Pack {
    pub fn load(root: impl AsRef<Path>, variant_uid: &VariantUID) -> Result<Self> {
        let path = root.as_ref();
        let manifest_path = path.join(manifest::FILENAME);
        let manifest = Manifest::load(&manifest_path)
            .with_context(|| eyre!("failed to load manifest at {manifest_path:?}"))?;

        Self::load_with_manifest(path, manifest, variant_uid)
    }

    pub fn load_with_manifest(
        root: impl Into<PathBuf>,
        manifest: Manifest,
        variant_uid: &VariantUID,
    ) -> Result<Self> {
        let root = root.into();
        let api = Api::new(&root, variant_uid).context("failed to create lua api")?;

        let init_path = root.join("scripts/init.lua");
        api.lua()
            .load(&*init_path)
            .exec()
            .with_context(|| eyre!("error executing {init_path:?}"))?;

        Ok(Self {
            root,
            manifest,
            api: api,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
#[serde(transparent)]
pub struct VariantUID(Arc<str>);

impl VariantUID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
