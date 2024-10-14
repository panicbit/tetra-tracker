use std::fs;
use std::path::{Path, PathBuf};

use egui::{CentralPanel, CollapsingHeader, ScrollArea, Ui, WidgetText};
use eyre::{Error, Result};
use tracing::error;

use crate::pack::{manifest, Manifest, Pack};

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

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Option<Pack> {
        let mut loaded_pack = None;

        CentralPanel::default().show(ctx, |ui| {
            let pack_manifests = self.get_or_load_manifests();

            show_packs(ui, pack_manifests, &mut loaded_pack)
        });

        loaded_pack
    }

    fn get_or_load_manifests(&mut self) -> &[(PathBuf, Manifest)] {
        let pack_manifests = &*self.pack_manifests.get_or_insert_with(|| {
            match try_read_pack_manifests(&self.packs_path) {
                Ok(pack_manifests) => pack_manifests,
                Err(err) => {
                    self.error = Some(err);
                    Vec::new()
                }
            }
        });
        pack_manifests
    }
}

fn show_packs(ui: &mut Ui, pack_manifests: &[(PathBuf, Manifest)], loaded_pack: &mut Option<Pack>) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.vertical(|ui| {
            for (manifest_path, manifest) in pack_manifests {
                let pack_name = WidgetText::from(&manifest.name).heading();

                CollapsingHeader::new(pack_name)
                    .default_open(false)
                    .show(ui, |ui| {
                        show_variants(ui, manifest, manifest_path, loaded_pack);
                    });

                ui.end_row();
            }
        });

        ui.allocate_space(ui.available_size());
    });
}

fn show_variants(
    ui: &mut Ui,
    manifest: &Manifest,
    manifest_path: &PathBuf,
    loaded_pack: &mut Option<Pack>,
) {
    ui.horizontal(|ui| {
        for (variant_id, variant) in &manifest.variants {
            if ui.button(&variant.display_name).clicked() {
                match Pack::load(manifest_path, variant_id) {
                    Ok(pack) => {
                        *loaded_pack = Some(pack);
                    }
                    Err(err) => error!("{err:?}"),
                }
            }

            ui.end_row();
        }
    });
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
                error!("{err:?}");
                continue;
            }
        };

        manifests.push((pack_path, manifest));
    }

    Ok(manifests)
}
