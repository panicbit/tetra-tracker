use std::env;

use clap::Parser;
use eyre::{Context, Result};
use tetra_tracker::cli::Cli;
use tetra_tracker::pack::{manifest, Manifest, Pack};
use tetra_tracker::ui::{self, PackPicker};
use tracing::level_filters::LevelFilter;
use tracing::{error, info, instrument};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

#[tracing::instrument(level = "trace", skip())]
fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing()?;

    let version = env!("CARGO_PKG_VERSION");
    info!(version, "Starting tetra-tracker");

    let cli = Cli::parse();
    let pack = try_load_pack_from_cli(&cli)
        .inspect_err(|err| error!("{err:?}"))
        .ok()
        .flatten();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Tetra Tracker",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, pack)))),
    )
    .expect("failed to run via eframe");

    Ok(())
}

fn init_tracing() -> Result<()> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env()?;

    let fmt_subscriber = tracing_subscriber::fmt::layer()
        .with_file(false)
        .with_line_number(false)
        .without_time()
        .with_target(false);

    let erorr_subcsriber = tracing_error::ErrorLayer::default();

    let subscriber = tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(fmt_subscriber)
        .with(erorr_subcsriber);

    tracing::subscriber::set_global_default(subscriber).context("failed to set global tracer")?;

    Ok(())
}

#[instrument(level = "trace")]
fn try_load_pack_from_cli(cli: &Cli) -> Result<Option<Pack>> {
    let Some(pack_path) = &cli.pack_path else {
        return Ok(None);
    };

    let manifest_path = pack_path.join(manifest::FILENAME);
    let manifest = Manifest::load(manifest_path)?;
    let maybe_variant = match &cli.variant {
        Some(requested_variant) => manifest.variants.iter().find(|(variant_uid, variant)| {
            requested_variant == variant_uid.as_str()
                || requested_variant == variant.display_name.as_str()
        }),
        None => manifest.variants.first(),
    };
    let Some((variant_id, _)) = maybe_variant else {
        return Ok(None);
    };

    let pack = Pack::load(pack_path, variant_id)?;

    Ok(Some(pack))
}

enum App {
    PackPicker(ui::PackPicker),
    Tracker(ui::Tracker),
}

impl App {
    #[instrument(skip_all)]
    fn new(cc: &eframe::CreationContext<'_>, pack: Option<Pack>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        match pack {
            None => {
                let pack_dir = env::current_dir().unwrap().join("packs");
                Self::PackPicker(ui::PackPicker::new(pack_dir))
            }
            Some(pack) => Self::Tracker(ui::Tracker::new(pack)),
        }
    }
}

impl eframe::App for App {
    #[instrument(skip_all, level = "trace")]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self {
            App::PackPicker(pack_picker) => {
                if let Some(pack) = pack_picker.update(ctx, frame) {
                    *self = Self::Tracker(ui::Tracker::new(pack));
                }
            }
            App::Tracker(tracker) => {
                if tracker.update(ctx, frame).is_break() {
                    let pack_dir = env::current_dir().unwrap().join("packs");
                    *self = Self::PackPicker(PackPicker::new(pack_dir))
                }
            }
        }
    }
}
