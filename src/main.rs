mod assets;
mod catalog;
mod commands;
mod contact_sheet;
mod installed;
mod installer;
mod palette;
mod presets;
mod toggle;

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
    /// Inventory player skin ramps and unclassified portrait colors from a local archive.
    Catalog {
        #[arg(long)]
        archive: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Build several named variants from one reviewed region profile.
    BuildPresets {
        #[arg(long)]
        original: PathBuf,
        #[arg(long)]
        presets: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Generate and install the Adeline toggle into an explicit, closed game copy.
    Install {
        #[arg(long)]
        game_dir: PathBuf,
        /// MOMI CLI executable; defaults to the release provided by the Nix shell.
        #[arg(long)]
        momi: Option<PathBuf>,
        /// Exact-color palette; defaults to the embedded blue study.
        #[arg(long)]
        palette: Option<PathBuf>,
        /// Named presets sharing a reviewed region profile.
        #[arg(long, conflicts_with = "palette")]
        presets: Option<PathBuf>,
        /// Current MOMI config/mods/manifest.json; required for a MOMI-modified archive.
        #[arg(long)]
        installed_mods: Option<PathBuf>,
    },
    /// Restore the exact archive saved before this tool installed the toggle.
    Uninstall {
        #[arg(long)]
        game_dir: PathBuf,
    },
    /// Copy up to 50 explicit PNG assets and their metadata from assets.zip.
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
        /// Also verify exact palette output, including region restrictions.
        #[arg(long)]
        palette: Option<PathBuf>,
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
        /// Add a third column highlighting changed pixels in magenta.
        #[arg(long)]
        changes: bool,
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
    /// Build an F6 toggle for locally generated Adeline spring and summer portraits.
    PackageToggle {
        #[arg(long)]
        original: PathBuf,
        #[arg(long)]
        modified: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
}

fn run() -> Result<()> {
    let report = match Cli::parse().command {
        Command::Catalog { archive, output } => catalog::build(&archive, &output)?,
        Command::BuildPresets {
            original,
            presets,
            output,
        } => presets::build(&original, &presets, &output)?,
        Command::Install {
            game_dir,
            momi,
            palette,
            presets,
            installed_mods,
        } => installer::install(
            &game_dir,
            momi.as_deref(),
            palette.as_deref(),
            presets.as_deref(),
            installed_mods.as_deref(),
        )?,
        Command::Uninstall { game_dir } => installer::uninstall(&game_dir)?,
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
        Command::Validate {
            original,
            modified,
            palette,
        } => commands::validate(&original, &modified, palette.as_deref())?,
        Command::ContactSheet {
            original,
            modified,
            output,
            zoom,
            changes,
        } => contact_sheet::build(&original, &modified, &output, zoom, changes)?,
        Command::Package {
            original,
            modified,
            manifest,
            output,
        } => commands::package(&original, &modified, manifest.as_deref(), &output)?,
        Command::PackageToggle {
            original,
            modified,
            output,
        } => toggle::package(&original, &modified, &output)?,
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
