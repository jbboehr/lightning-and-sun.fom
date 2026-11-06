//! Snapshot missing world assets and render local source evidence for material review.
use anyhow::{Context, Result, ensure};
use clap::Parser;
use image::{Rgba, RgbaImage};
use rc_zip_sync::ReadZip;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, collections::BTreeSet, fs, path::PathBuf};

#[derive(Parser)]
struct Args {
    archive: PathBuf,
    output: PathBuf,
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn main() -> Result<()> {
    let args = Args::parse();
    ensure!(!args.output.exists(), "Choose a fresh output directory");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut inputs = BTreeMap::new();
    let mut read = |path: PathBuf| -> Result<Value> {
        let path = path.canonicalize()?;
        let bytes = fs::read(&path)?;
        inputs.insert(
            path.strip_prefix(&root)?.to_string_lossy().into_owned(),
            digest(&bytes),
        );
        Ok(serde_json::from_slice(&bytes)?)
    };
    let registry = read(root.join("mod/config/characters.json"))?;
    let selected = read(root.join("palettes/sets/characters-trial.json"))?;
    let file = fs::File::open(&args.archive)?;
    let archive = file.read_zip()?;
    let mut names = BTreeSet::new();
    for entry in archive.entries() {
        ensure!(names.insert(entry.name.clone()), "Duplicate archive member");
    }
    let mut rows = Vec::new();
    let mut totals = BTreeMap::new();
    let mut previews = Vec::new();
    for character in selected["characters"]
        .as_array()
        .context("Character list")?
    {
        let id = character["id"].as_str().context("Character ID")?;
        let registered = registry
            .as_array()
            .context("Registry")?
            .iter()
            .find(|entry| entry["id"] == id)
            .context("Missing registered character")?;
        let label = registered["label"].as_str().context("Character label")?;
        let set_path = root
            .join("palettes/sets")
            .join(character["presets"].as_str().context("Preset set path")?);
        let set = read(set_path.clone())?;
        let profile = read(
            set_path
                .parent()
                .unwrap()
                .join(set["profile"].as_str().context("Profile path")?),
        )?;
        let regions = profile["regions"].as_array().context("Profile regions")?;
        let covered: BTreeSet<_> = regions
            .iter()
            .map(|region| region["asset"].as_str().context("Region asset"))
            .collect::<Result<_>>()?;
        ensure!(covered.len() == regions.len(), "Duplicate profile asset");
        let prefix = format!("assets/animations/NPCs/{label}/Sprites/");
        for name in names
            .iter()
            .filter(|n| n.starts_with(&prefix) && n.ends_with(".png"))
            .filter(|n| !covered.contains(n.as_str()))
        {
            let bytes = archive.by_name(name).context("Missing PNG")?.bytes()?;
            let image = image::load_from_memory(&bytes)?.to_rgba8();
            let meta_name = format!("{}.meta.toml", name.trim_end_matches(".png"));
            let meta = archive
                .by_name(&meta_name)
                .context("Missing metadata")?
                .bytes()?;
            let metadata: toml::Value = toml::from_str(std::str::from_utf8(&meta)?)?;
            let props = &metadata["asset_properties"];
            let size = props["frame_size"].as_array().context("Frame size")?;
            let w = size[0].as_integer().context("Frame width")? as u32;
            let h = size[1].as_integer().context("Frame height")? as u32;
            let frames = props
                .get("frame_len")
                .and_then(|v| v.as_integer())
                .unwrap_or(1) as u32;
            ensure!(w > 0 && h > 0 && frames > 0, "Invalid frame dimensions");
            ensure!(
                Some(image.width()) == w.checked_mul(frames) && image.height() == h,
                "Strip dimensions differ from metadata: {name}"
            );
            let shadow = (id == "hayden" && name.contains("/Shadow/"))
                || (id == "balor" && name.ends_with("shadow_jump_east.png"));
            let spectral = id == "seridia" && name.contains("/Priestess/");
            let mut colors = BTreeMap::new();
            if shadow || spectral {
                for p in image.pixels().filter(|p| p[3] > 0) {
                    *colors
                        .entry(format!("#{:02X}{:02X}{:02X}{:02X}", p[0], p[1], p[2], p[3]))
                        .or_insert(0usize) += 1;
                }
            }
            if shadow {
                ensure!(
                    image
                        .pixels()
                        .all(|p| p[3] == 0 || p.0 == [37, 43, 67, 255]),
                    "Shadow audit needs a new material decision: {name}"
                );
            }
            let disposition = if shadow {
                "unchanged_shadow"
            } else if spectral {
                "unresolved_spectral_form"
            } else {
                "recolor_review"
            };
            *totals.entry(disposition).or_insert(0usize) += 1;
            rows.push(
                json!({"character":id,"asset":name,"disposition":disposition,
                "png_sha256":digest(&bytes),"metadata_sha256":digest(&meta),
                "frame_size":[w,h],"frames":frames,"audit_colors":colors}),
            );
            if spectral || (shadow && id == "balor") {
                previews.push((
                    format!("special-{}.png", previews.len()),
                    image,
                    w,
                    h,
                    frames,
                    spectral,
                ));
            }
        }
    }
    let mut hash = Sha256::new();
    std::io::copy(&mut fs::File::open(&args.archive)?, &mut hash)?;
    let report = json!({"archive_sha256":format!("{:x}",hash.finalize()),
        "inputs":inputs,"totals":totals,"assets":rows});
    fs::create_dir(&args.output)?;
    fs::write(
        args.output.join("inventory.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    let spectral_frames: u32 = previews.iter().filter(|p| p.5).map(|p| p.4).sum();
    let mut spectral_sheet = RgbaImage::from_pixel(
        1200,
        240 * spectral_frames.div_ceil(5),
        Rgba([35, 40, 48, 255]),
    );
    let mut cell = 0;
    for (name, image, w, h, frames, spectral) in previews {
        let mut sheet = RgbaImage::from_pixel(
            w * 8 * 3,
            h * frames.div_ceil(8) * 3,
            Rgba([35, 40, 48, 255]),
        );
        for frame in 0..frames {
            let crop = image::imageops::crop_imm(&image, frame * w, 0, w, h).to_image();
            let enlarged =
                image::imageops::resize(&crop, w * 3, h * 3, image::imageops::FilterType::Nearest);
            image::imageops::overlay(
                &mut sheet,
                &enlarged,
                i64::from(frame % 8 * w * 3),
                i64::from(frame / 8 * h * 3),
            );
            if spectral {
                ensure!(
                    (w, h) == (80, 80),
                    "Spectral audit montage expects 80-pixel frames"
                );
                image::imageops::overlay(
                    &mut spectral_sheet,
                    &enlarged,
                    i64::from(cell % 5 * 240),
                    i64::from(cell / 5 * 240),
                );
                cell += 1;
            }
        }
        sheet.save(args.output.join(name))?;
    }
    if spectral_frames > 0 {
        spectral_sheet.save(args.output.join("spectral-frames.png"))?;
    }
    println!(
        "{} omitted world strips: {}",
        rows.len(),
        serde_json::to_string(&totals)?
    );
    Ok(())
}
