mod manifest;

use std::path::Path;

use eyre::{eyre, Context, Result};
pub use manifest::Manifest;

pub struct Pack {
    pub manifest: Manifest,
}

impl Pack {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let manifest_path = path.join("manifest.json");
        let manifest = Manifest::load(&manifest_path)
            .with_context(|| eyre!("failed to load manifest at {manifest_path:?}"))?;

        Self::load_with_manifest(path, manifest)
    }

    pub fn load_with_manifest(path: impl AsRef<Path>, manifest: Manifest) -> Result<Self> {
        Ok(Self {
            manifest,
        })
    }
}
