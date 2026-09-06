mod assets;
mod commands;
mod contact_sheet;
mod palette;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about = "Offline, exact-color palette tools for Fields of Mistria"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Copy one or two explicit PNG assets and their metadata from assets.zip.
    Export {
        #[arg(long)]
        archive: PathBuf,
        #[arg(long, required = true)]
        asset: Vec<String>,
        #[arg(long)]
        output: PathBuf,
    },
    /// Apply simultaneous RGBA replacements into a fresh directory.
    Apply {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        palette: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Check filenames, dimensions, alpha, and unchanged metadata.
    Validate {
        #[arg(long)]
        original: PathBuf,
        #[arg(long)]
        modified: PathBuf,
    },
    /// Draw labeled original/modified pairs at 4x or 8x nearest-neighbor zoom.
    ContactSheet {
        #[arg(long)]
        original: PathBuf,
        #[arg(long)]
        modified: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value_t = 4)]
        zoom: u32,
    },
    /// Package at most two changed strips for MOMI; print the report to stdout.
    Package {
        #[arg(long)]
        original: PathBuf,
        #[arg(long)]
        modified: PathBuf,
        /// Custom manifest; defaults to the manifest embedded at build time.
        #[arg(long)]
        manifest: Option<PathBuf>,
        #[arg(long)]
        output: PathBuf,
    },
}

fn run() -> Result<()> {
    let report = match Cli::parse().command {
        Command::Export {
            archive,
            asset,
            output,
        } => commands::export(&archive, &asset, &output)?,
        Command::Apply {
            input,
            palette,
            output,
        } => commands::apply(&input, &palette, &output)?,
        Command::Validate { original, modified } => assets::compare(&original, &modified)?,
        Command::ContactSheet {
            original,
            modified,
            output,
            zoom,
        } => contact_sheet::build(&original, &modified, &output, zoom)?,
        Command::Package {
            original,
            modified,
            manifest,
            output,
        } => commands::package(&original, &modified, manifest.as_deref(), &output)?,
    };
    use std::io::Write;
    std::io::stdout()
        .lock()
        .write_all(&assets::json_bytes(&report)?)?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}
