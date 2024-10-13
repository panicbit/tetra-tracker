use std::fs;
use std::os::windows::fs::FileTypeExt;
use std::path::{Path, PathBuf};

use egui::{CentralPanel, Ui, Widget};
use eyre::{Error, Result};

use crate::pack::{Manifest, Pack};

pub struct PackPicker {
    packs_path: PathBuf,
    pack_manifests: Option<Vec<(PathBuf, Manifest)>>,
    error: Option<Error>,
}

impl PackPicker {
    pub fn new(packs_path: impl Into<PathBuf>) -> Self {
        Self {
            packs_path: packs_path.into(),
            pack_manifests: None,
            error: None,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Option<Pack> {
        let mut pack = None;

        CentralPanel::default().show(ctx, |ui| {
            let pack_manifests = self.pack_manifests.get_or_insert_with(|| {
                match try_read_pack_manifests(&self.packs_path) {
                    Ok(pack_manifests) => pack_manifests,
                    Err(err) => {
                        self.error = Some(err);
                        Vec::new()
                    }
                }
            });

            ui.vertical(|ui| {
                for (manifest_path, manifest) in pack_manifests {
                    if ui.button(&manifest.name).clicked() {
                        match Pack::load(manifest_path) {
                            Ok(loaded_pack) => {
                                pack = Some(loaded_pack);
                                ctx.request_repaint();
                            }
                            Err(err) => eprintln!("{err:#?}"),
                        }
                    }
                }
            });
        });

        pack
    }
}

fn try_read_pack_manifests(packs_path: &Path) -> Result<Vec<(PathBuf, Manifest)>> {
    let mut manifests = Vec::new();
    let entries = fs::read_dir(packs_path)?;

    for entry in entries {
        let entry = entry?;
        let metadata = fs::metadata(entry.path())?;

        if !metadata.is_dir() {
            continue;
        }

        let pack_path = entry.path().to_path_buf();
        let manifest_path = pack_path.join("manifest.json");
        let manifest = match Manifest::load(&manifest_path) {
            Ok(manifest) => manifest,
            Err(err) => {
                eprintln!("{err:#?}");
                continue;
            }
        };

        manifests.push((pack_path, manifest));
    }

    Ok(manifests)
}
