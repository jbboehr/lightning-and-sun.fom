mod assets;
mod catalog;
mod characters;
mod commands;
mod contact_sheet;
mod installed;
mod installer;
mod palette;
mod presets;
mod review;
mod toggle;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

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
    /// Build a local package containing independently selectable characters.
    BuildCharacters {
        #[arg(long)]
        archive: PathBuf,
        #[arg(long)]
        characters: PathBuf,
        #[arg(long)]
        output: PathBuf,
        /// Use existing masks despite changed source PNG hashes; keep other checks.
        #[arg(long)]
        allow_source_hash_mismatch: bool,
    },
    /// Generate a local mask review gallery with exact-frame reuse and component suggestions.
    ReviewBatch {
        #[arg(long)]
        archive: PathBuf,
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
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
        /// Use existing masks despite changed source PNG hashes; keep other checks.
        #[arg(long)]
        allow_source_hash_mismatch: bool,
    },
    /// Generate and install palette toggles into an explicit, closed game copy.
    Install {
        #[arg(long)]
        game_dir: PathBuf,
        /// MOMI CLI executable; defaults to the release provided by the Nix shell.
        #[arg(long)]
        momi: Option<PathBuf>,
        /// Exact-color palette for the single-portrait blue study.
        #[arg(long)]
        palette: Option<PathBuf>,
        /// Named presets sharing a reviewed region profile.
        #[arg(long, conflicts_with = "palette")]
        presets: Option<PathBuf>,
        /// Character selection and per-character preset sets; defaults to the packaged collection.
        #[arg(long, conflicts_with_all = ["palette", "presets"])]
        characters: Option<PathBuf>,
        /// Current MOMI config/mods/manifest.json; required for a MOMI-modified archive.
        #[arg(long)]
        installed_mods: Option<PathBuf>,
        /// Refuse source PNGs whose hashes differ from the recipes instead of warning.
        #[arg(long)]
        strict_source_hashes: bool,
    },
    /// Restore the exact archive saved before this tool installed the toggle.
    Uninstall {
        #[arg(long)]
        game_dir: PathBuf,
    },
    /// Copy up to 512 explicit PNG assets and their metadata from assets.zip.
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
        /// Use existing masks despite changed source PNG hashes; keep other checks.
        #[arg(long)]
        allow_source_hash_mismatch: bool,
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
        /// Check palette output despite changed source PNG hashes; keep other checks.
        #[arg(long, requires = "palette")]
        allow_source_hash_mismatch: bool,
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
    /// Build a toggle for one character's locally generated, reviewed animations.
    PackageToggle {
        #[arg(long)]
        original: PathBuf,
        #[arg(long)]
        modified: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
}

fn packaged_collection() -> Result<PathBuf> {
    let path = std::env::current_exe()?
        .parent()
        .and_then(Path::parent)
        .map(|prefix| prefix.join("share/mistria-palette/palettes/sets/characters-trial.json"))
        .filter(|path| path.is_file())
        .context(
            "No packaged collection beside this executable; pass --characters, --presets or --palette",
        )?;
    eprintln!("Installing the complete collection from {}", path.display());
    Ok(path)
}

fn run() -> Result<()> {
    let report = match Cli::parse().command {
        Command::BuildCharacters {
            archive,
            characters,
            output,
            allow_source_hash_mismatch,
        } => characters::build(&archive, &characters, &output, allow_source_hash_mismatch)?,
        Command::ReviewBatch {
            archive,
            config,
            output,
        } => review::build(&archive, &config, &output)?,
        Command::Catalog { archive, output } => catalog::build(&archive, &output)?,
        Command::BuildPresets {
            original,
            presets,
            output,
            allow_source_hash_mismatch,
        } => presets::build(&original, &presets, &output, allow_source_hash_mismatch)?,
        Command::Install {
            game_dir,
            momi,
            palette,
            presets,
            characters,
            installed_mods,
            strict_source_hashes,
        } => installer::install(
            &game_dir,
            momi.as_deref(),
            palette.as_deref(),
            presets.as_deref(),
            match (&palette, &presets, characters) {
                (None, None, None) => Some(packaged_collection()?),
                (_, _, characters) => characters,
            }
            .as_deref(),
            installed_mods.as_deref(),
            !strict_source_hashes,
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
            allow_source_hash_mismatch,
        } => commands::apply(&input, &palette, &output, allow_source_hash_mismatch)?,
        Command::Validate {
            original,
            modified,
            palette,
            allow_source_hash_mismatch,
        } => commands::validate(
            &original,
            &modified,
            palette.as_deref(),
            allow_source_hash_mismatch,
        )?,
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
