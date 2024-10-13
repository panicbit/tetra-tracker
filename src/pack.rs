use std::path::{Path, PathBuf};

use eyre::{eyre, Context, Result};
pub use manifest::Manifest;
use mlua::Lua;

use crate::pack::api::Api;

pub mod api;
mod manifest;

pub struct Pack {
    pub root: PathBuf,
    pub manifest: Manifest,
    pub api: Api,
}

impl Pack {
    pub fn load(root: impl AsRef<Path>) -> Result<Self> {
        let path = root.as_ref();
        let manifest_path = path.join("manifest.json");
        let manifest = Manifest::load(&manifest_path)
            .with_context(|| eyre!("failed to load manifest at {manifest_path:?}"))?;

        Self::load_with_manifest(path, manifest)
    }

    pub fn load_with_manifest(root: impl Into<PathBuf>, manifest: Manifest) -> Result<Self> {
        let root = root.into();
        let api = Api::new(&root).context("failed to create lua api")?;

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
