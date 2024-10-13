use std::fs;
use std::os::windows::fs::FileTypeExt;
use std::path::{Path, PathBuf};

use egui::{CentralPanel, Ui, Widget};
use eyre::{Error, Result};

use crate::pack::{manifest, Manifest, Pack};

pub struct PackPicker {
    packs_path: PathBuf,
    pack_manifests: Option<Vec<(PathBuf, Manifest)>>,
    selected_manifest: usize,
    error: Option<Error>,
}

impl PackPicker {
    pub fn new(packs_path: impl Into<PathBuf>) -> Self {
        Self {
            packs_path: packs_path.into(),
            pack_manifests: None,
            selected_manifest: 0,
            error: None,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Option<Pack> {
        let mut pack = None;

        CentralPanel::default().show(ctx, |ui| {
            let pack_manifests = &*self.pack_manifests.get_or_insert_with(|| {
                match try_read_pack_manifests(&self.packs_path) {
                    Ok(pack_manifests) => pack_manifests,
                    Err(err) => {
                        self.error = Some(err);
                        Vec::new()
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        for (manifest_index, (_, manifest)) in pack_manifests.iter().enumerate() {
                            if ui.button(&manifest.name).clicked() {
                                self.selected_manifest = manifest_index;
                            }
                        }
                    });
                });

                ui.group(|ui| {
                    let Some((manifest_path, manifest)) =
                        pack_manifests.get(self.selected_manifest)
                    else {
                        return;
                    };

                    ui.vertical(|ui| {
                        for (variant_id, variant) in &manifest.variants {
                            if ui.button(&variant.display_name).clicked() {
                                match Pack::load(manifest_path, variant_id) {
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
        let manifest_path = pack_path.join(manifest::FILENAME);
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
