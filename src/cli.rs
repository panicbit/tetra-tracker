use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Cli {
    /// path to a poptracker pack
    pub pack_path: Option<PathBuf>,
    pub variant: Option<String>,
}
