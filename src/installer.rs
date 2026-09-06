use crate::{assets::file_digest, commands, installed, toggle};
use anyhow::{Context, Result, ensure};
use rc_zip_sync::ReadZip;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const STATE: &str = ".mistria-palette";
#[derive(Serialize, Deserialize)]
struct Receipt {
    version: u32,
    previous_sha256: String,
    installed_sha256: String,
}

#[derive(Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct ModSelection {
    name: String,
    version: String,
}
#[derive(Deserialize)]
struct InstalledMods {
    mods: Vec<ModSelection>,
}

fn read_installed_mods(path: &Path) -> Result<Vec<ModSelection>> {
    let installed: InstalledMods = serde_json::from_slice(&fs::read(path)?)?;
    Ok(installed.mods)
}

fn selected_mods(root: &Path) -> Result<Vec<ModSelection>> {
    let mut selected: Vec<ModSelection> = Vec::new();
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if !path.is_dir() {
            ensure!(!path.extension().is_some_and(|s| s.eq_ignore_ascii_case("zip") || s.eq_ignore_ascii_case("rar")), "Unpack archive mods into folders before using this prototype");
            continue;
        }
        let json = path.join("manifest.json");
        let selection = if json.is_file() {
            serde_json::from_slice(&fs::read(json)?)?
        } else {
            toml::from_str(
                &fs::read_to_string(path.join("manifest.toml")).with_context(|| {
                    format!("Expected a mod manifest directly inside {}", path.display())
                })?,
            )?
        };
        selected.push(selection);
    }
    selected.sort();
    ensure!(
        selected.windows(2).all(|pair| pair[0].name != pair[1].name),
        "Duplicate mod names are unsupported by this prototype"
    );
    Ok(selected)
}

fn regular(path: &Path) -> Result<()> {
    ensure!(
        fs::symlink_metadata(path)?.file_type().is_file(),
        "Expected a regular file: {}",
        path.display()
    );
    Ok(())
}
fn game(path: &Path) -> Result<(PathBuf, fs::File)> {
    ensure!(
        cfg!(all(target_os = "linux", target_arch = "x86_64")),
        "The installer prototype supports x86_64 Linux"
    );
    let game = fs::canonicalize(path)?;
    regular(&game.join("assets.zip"))?;
    regular(&game.join("Maybe.toml"))?;
    let lock_path = game.join(".mistria-palette.lock");
    if lock_path.symlink_metadata().is_ok() {
        regular(&lock_path)?;
    }
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)?;
    lock.try_lock()
        .context("Another palette operation is using this game directory")?;
    Ok((game, lock))
}
fn mods_dir(game: &Path) -> Result<Option<PathBuf>> {
    let dirs: Vec<_> = [game.join("mods"), game.join("Mods")]
        .into_iter()
        .filter(|p| p.symlink_metadata().is_ok())
        .collect();
    ensure!(
        dirs.len() <= 1,
        "Both mods and Mods exist; choose one directory before installing"
    );
    if let Some(dir) = dirs.first() {
        ensure!(
            fs::symlink_metadata(dir)?.file_type().is_dir(),
            "Mods must be a real directory"
        );
    }
    Ok(dirs.into_iter().next())
}
fn copy_mods(source: &Path, target: &Path) -> Result<()> {
    for entry in walkdir::WalkDir::new(source) {
        let entry = entry?;
        let kind = entry.file_type();
        ensure!(
            kind.is_file() || kind.is_dir(),
            "Mod links and special files are unsupported: {}",
            entry.path().display()
        );
        let output = target.join(entry.path().strip_prefix(source)?);
        if kind.is_dir() {
            fs::create_dir_all(output)?;
        } else {
            fs::copy(entry.path(), output)?;
        }
    }
    Ok(())
}
fn pristine(game: &Path, installed_mods: Option<&Path>) -> Result<PathBuf> {
    let live = game.join("assets.zip");
    let file = fs::File::open(&live)?;
    let archive = file.read_zip()?;
    if archive.by_name("manifest.toml").is_none() {
        return Ok(live);
    }
    ensure!(
        installed_mods.is_some(),
        "MOMI-modified archive requires --installed-mods with its current config/mods/manifest.json"
    );
    let backup = game.join("assets.bak.zip");
    regular(&backup).context("MOMI-modified archive needs its matching assets.bak.zip")?;
    let backup_file = fs::File::open(&backup)?;
    ensure!(
        backup_file.read_zip()?.by_name("manifest.toml").is_none(),
        "MOMI backup is itself modified"
    );
    Ok(backup)
}
fn sync(path: &Path) -> Result<()> {
    fs::File::open(path)?.sync_all()?;
    Ok(())
}

pub fn install(
    path: &Path,
    momi: Option<&Path>,
    palette: Option<&Path>,
    installed_mods: Option<&Path>,
) -> Result<Value> {
    let (game, _lock) = game(path)?;
    let state = game.join(STATE);
    ensure!(
        state.symlink_metadata().is_err(),
        "Palette state already exists; run uninstall before rebuilding"
    );
    let runner = std::env::var_os("MISTRIA_MOMI_RUNNER")
        .or_else(|| option_env!("MISTRIA_MOMI_RUNNER").map(Into::into))
        .context("Build in the Nix dev shell to provide the isolated MOMI runner")?;
    let momi = momi
        .map(PathBuf::from)
        .or_else(|| {
            option_env!("MISTRIA_MOMI_BINARY")
                .filter(|p| !p.is_empty())
                .map(PathBuf::from)
        })
        .context("Supply --momi or build in the x86_64 Linux Nix shell")?;
    let momi = fs::canonicalize(momi)?;
    regular(&momi)?;
    let source = pristine(&game, installed_mods)?;
    let previous_sha256 = file_digest(&game.join("assets.zip"))?;
    let work = tempfile::Builder::new()
        .prefix(".mistria-palette-")
        .tempdir_in(&game)?;
    fs::copy(game.join("assets.zip"), work.path().join("previous.zip"))?;
    let build = work.path().join("build");
    fs::create_dir(&build)?;
    fs::copy(&source, build.join("assets.zip"))?;
    fs::copy(game.join("Maybe.toml"), build.join("Maybe.toml"))?;
    let mods = build.join("mods");
    fs::create_dir(&mods)?;
    if let Some(existing) = mods_dir(&game)? {
        copy_mods(&existing, &mods)?;
    }
    let prior_mods = installed_mods.map(read_installed_mods).transpose()?;
    if let Some(prior) = &prior_mods {
        let mut selection = prior.clone();
        selection.sort();
        ensure!(
            selection == selected_mods(&mods)?,
            "Existing mod sources do not match --installed-mods; restore the matching mod folders before installing"
        );
    }
    ensure!(
        !mods.join("lns_palette").exists(),
        "Remove the separately installed lns_palette package first"
    );
    let original = build.join("original");
    let modified = build.join("modified");
    eprintln!("Generating the portrait from local game assets...");
    commands::export(
        &build.join("assets.zip"),
        &[format!("{}.png", installed::SOURCE)],
        &original,
    )?;
    let palette_file = build.join("palette.json");
    fs::write(
        &palette_file,
        match palette {
            Some(path) => fs::read(path)?,
            None => include_bytes!("../palettes/stylized/adeline.json").to_vec(),
        },
    )?;
    let report = commands::apply(&original, &palette_file, &modified)?;
    toggle::package(&original, &modified, &mods.join("lns_palette"))?;
    let expected_mods = selected_mods(&mods)?;
    eprintln!("Building an isolated MOMI archive...");
    let result = Command::new(runner)
        .arg(&momi)
        .arg(&build)
        .output()
        .context("Could not start the isolated MOMI runner")?;
    ensure!(
        result.status.success(),
        "MOMI failed ({}):\n{}\n{}",
        result.status,
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    eprintln!("Verifying installed portrait pixels and animation metadata...");
    let installed_mods = read_installed_mods(&build.join("config/mods/manifest.json"))
        .context("MOMI did not report its installed mod selection")?;
    let mut installed_selection = installed_mods.clone();
    installed_selection.sort();
    ensure!(
        installed_selection == expected_mods,
        "MOMI skipped or changed the selected mods; nothing was published"
    );
    if let Some(prior) = &prior_mods {
        // The new palette can occur anywhere, but existing mod precedence must stay intact.
        ensure!(
            installed_mods
                .iter()
                .filter(|entry| prior.contains(entry))
                .eq(prior.iter()),
            "MOMI changed the existing mod load order; nothing was published. This CLI cannot preserve that custom order; use a MOMI workflow that preserves it"
        );
    }
    installed::verify(&build.join("assets.zip"), &original, &modified)?;
    let installed_sha256 = file_digest(&build.join("assets.zip"))?;
    ensure!(
        file_digest(&game.join("assets.zip"))? == previous_sha256,
        "Game archive changed during installation; nothing was published"
    );
    ensure!(
        file_digest(&work.path().join("previous.zip"))? == previous_sha256,
        "Backup does not match the original archive"
    );
    fs::rename(build.join("assets.zip"), work.path().join("ready.zip"))?;
    fs::remove_dir_all(&build)?;
    let receipt = Receipt {
        version: 1,
        previous_sha256,
        installed_sha256: installed_sha256.clone(),
    };
    fs::write(
        work.path().join("receipt.json"),
        serde_json::to_vec_pretty(&receipt)?,
    )?;
    for name in ["previous.zip", "ready.zip", "receipt.json"] {
        sync(&work.path().join(name))?;
    }
    sync(work.path())?;
    // Publish recovery information before the single atomic archive replacement.
    fs::rename(work.path(), &state)?;
    sync(&game)?;
    fs::rename(state.join("ready.zip"), game.join("assets.zip"))
        .context("Could not publish archive; run uninstall to clear the prepared installation")?;
    sync(&game)?;
    sync(&state)?;
    Ok(
        json!({"installed": true, "game_dir": game, "archive_sha256": installed_sha256, "palette": report, "hotkey": "F6"}),
    )
}

pub fn uninstall(path: &Path) -> Result<Value> {
    let (game, _lock) = game(path)?;
    let state = game.join(STATE);
    if state.symlink_metadata().is_err() {
        return Ok(json!({"installed": false, "changed": false, "game_dir": game}));
    }
    ensure!(
        fs::symlink_metadata(&state)?.file_type().is_dir(),
        "Palette state must be a real directory"
    );
    if fs::read_dir(&state)?.next().is_none() {
        fs::remove_dir(&state)?;
        sync(&game)?;
        return Ok(json!({"installed": false, "changed": true, "game_dir": game}));
    }
    for entry in fs::read_dir(&state)? {
        let entry = entry?;
        ensure!(
            ["previous.zip", "receipt.json", "ready.zip"]
                .iter()
                .any(|n| entry.file_name() == *n),
            "Unexpected file in palette recovery directory"
        );
        regular(&entry.path())?;
    }
    let receipt: Receipt = serde_json::from_slice(&fs::read(state.join("receipt.json"))?)?;
    ensure!(receipt.version == 1, "Unsupported palette receipt version");
    let live_hash = file_digest(&game.join("assets.zip"))?;
    if live_hash != receipt.previous_sha256 {
        ensure!(
            live_hash == receipt.installed_sha256,
            "Game archive changed since installation. Refusing to overwrite an update or another mod operation; recovery files were retained"
        );
        ensure!(
            file_digest(&state.join("previous.zip"))? == receipt.previous_sha256,
            "Recovery archive checksum mismatch"
        );
        let restore = tempfile::Builder::new()
            .prefix(".mistria-palette-restore-")
            .tempfile_in(&game)?;
        fs::copy(state.join("previous.zip"), restore.path())?;
        sync(restore.path())?;
        restore.persist(game.join("assets.zip"))?;
        sync(&game)?;
    }
    // Also clears an interrupted install that prepared a receipt but never published.
    for name in ["previous.zip", "ready.zip"] {
        let path = state.join(name);
        if path.exists() {
            fs::remove_file(path)?;
        }
    }
    fs::remove_file(state.join("receipt.json"))?;
    fs::remove_dir(&state)?;
    sync(&game)?;
    Ok(
        json!({"installed": false, "changed": true, "game_dir": game, "archive_sha256": receipt.previous_sha256}),
    )
}
