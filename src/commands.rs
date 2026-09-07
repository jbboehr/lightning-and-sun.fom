use crate::{assets::*, palette};
use anyhow::{Context, Result, ensure};
use image::DynamicImage;
use rc_zip_sync::ReadZip;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

pub fn apply(input: &Path, palette_path: &Path, output: &Path) -> Result<Value> {
    let output = fresh_output(output, &[input, palette_path])?;
    let palette = palette::load(palette_path)?;
    apply_palette(input, &palette, &output)
}

pub fn apply_palette(input: &Path, palette: &palette::Palette, output: &Path) -> Result<Value> {
    let output = fresh_output(output, &[input])?;
    let (images, metadata) = inventory(input)?;
    palette.check_inventory(&images)?;
    let mut outputs = Outputs::new();
    let mut rows = Vec::new();
    let mut total = 0u64;
    for (name, path) in images {
        let before = fs::read(path)?;
        let mut image = rgba(&before).with_context(|| name.clone())?;
        let mask = palette.mask(&name, &before, &image)?;
        let mut counts = BTreeMap::<String, u64>::new();
        let mut excluded = 0u64;
        for (index, pixel) in image.pixels_mut().enumerate() {
            if let Some(target) = palette
                .mapping
                .get(&pixel.0)
                .filter(|target| **target != pixel.0)
            {
                if mask.as_ref().is_some_and(|mask| !mask[index]) {
                    excluded += 1;
                    continue;
                }
                let [r, g, b, a] = pixel.0;
                *counts
                    .entry(format!("#{r:02X}{g:02X}{b:02X}{a:02X}"))
                    .or_default() += 1;
                pixel.0 = *target;
            }
        }
        let size = [image.width(), image.height()];
        let changed: u64 = counts.values().sum();
        let after = if changed == 0 {
            before.clone()
        } else {
            png(&DynamicImage::ImageRgba8(image))?
        };
        total += changed;
        let mut row = json!({"path":name,"size":size,"changed_pixels":changed,"changed_by_source":counts,
            "original_sha256":digest(&before),"modified_sha256":digest(&after)});
        if let Some(mask) = mask {
            row["selected_pixels"] = json!(mask.iter().filter(|&&selected| selected).count());
            row["excluded_matching_pixels"] = json!(excluded);
        }
        rows.push(row);
        outputs.insert(name, after);
    }
    for (name, path) in &metadata {
        outputs.insert(name.clone(), fs::read(path)?);
    }
    let report = json!({"files":rows,"changed_pixels":total,"metadata_files":metadata.keys().collect::<Vec<_>>()});
    outputs.insert("palette-report.json".into(), json_bytes(&report)?);
    write_tree(&output, outputs)?;
    Ok(report)
}

pub fn validate(original: &Path, modified: &Path, palette_path: Option<&Path>) -> Result<Value> {
    let mut report = compare(original, modified)?;
    if let Some(path) = palette_path {
        let palette = palette::load(path)?;
        let (images, _) = inventory(original)?;
        palette.check_inventory(&images)?;
        for (name, path) in images {
            let bytes = fs::read(path)?;
            let before = rgba(&bytes)?;
            let after = rgba(&fs::read(modified.join(&name))?)?;
            let mask = palette.mask(&name, &bytes, &before)?;
            for (index, (left, right)) in before.pixels().zip(after.pixels()).enumerate() {
                let expected = if mask.as_ref().is_none_or(|mask| mask[index]) {
                    palette.mapping.get(&left.0).unwrap_or(&left.0)
                } else {
                    &left.0
                };
                ensure!(
                    right.0 == *expected,
                    "Palette mismatch: {name} at [{},{}]",
                    index % before.width() as usize,
                    index / before.width() as usize
                );
            }
        }
        report["palette_verified"] = json!(true);
    }
    Ok(report)
}

pub fn export(archive_path: &Path, assets: &[String], output: &Path) -> Result<Value> {
    let output = fresh_output(output, &[archive_path])?;
    let names: BTreeSet<_> = assets.iter().collect();
    ensure!(
        (1..=100).contains(&assets.len()) && names.len() == assets.len(),
        "Select between one and 100 distinct PNG assets"
    );
    let file = fs::File::open(archive_path)?;
    let archive = file.read_zip()?;
    let mut outputs = Outputs::new();
    let mut rows = Vec::new();
    for name in names {
        ensure!(
            name.starts_with("assets/")
                && name.ends_with(".png")
                && !name.contains('\\')
                && name
                    .split('/')
                    .all(|part| !part.is_empty() && part != "." && part != ".."),
            "Expected an exact, relative assets/...png member: {name}"
        );
        let metadata = format!("{}.meta.toml", name.strip_suffix(".png").unwrap());
        for member in [name, &metadata] {
            let mut matches = archive.entries().filter(|entry| entry.name == *member);
            let entry = matches
                .next()
                .with_context(|| format!("Missing archive member: {member}"))?;
            ensure!(
                matches.next().is_none(),
                "Duplicate archive member: {member}"
            );
            outputs.insert(member.clone(), entry.bytes()?);
        }
        let image = rgba(&outputs[name]).with_context(|| name.clone())?;
        rows.push(json!({"path":name,"size":[image.width(),image.height()],"sha256":digest(&outputs[name]),
            "metadata_sha256":digest(&outputs[&metadata])}));
    }
    let report = json!({"archive_sha256":file_digest(archive_path)?,"files":rows});
    outputs.insert("export-report.json".into(), json_bytes(&report)?);
    write_tree(&output, outputs)?;
    Ok(report)
}

#[derive(Deserialize)]
struct Metadata {
    asset_properties: Frames,
}
#[derive(Deserialize)]
struct Frames {
    frame_size: [u32; 2],
    #[serde(default = "one")]
    frame_len: u32,
}
fn one() -> u32 {
    1
}

pub fn package(
    original: &Path,
    modified: &Path,
    manifest_path: Option<&Path>,
    output: &Path,
) -> Result<Value> {
    let mut inputs = vec![original, modified];
    if let Some(path) = manifest_path {
        inputs.push(path);
    }
    let output = fresh_output(output, &inputs)?;
    let manifest = match manifest_path {
        Some(path) => fs::read(path)?,
        None => include_bytes!("../mod/manifest.toml").to_vec(),
    };
    let fields: toml::Value = toml::from_str(std::str::from_utf8(&manifest)?)?;
    for name in [
        "name",
        "author",
        "version",
        "minInstallerVersion",
        "manifestVersion",
    ] {
        ensure!(
            fields
                .get(name)
                .and_then(toml::Value::as_str)
                .is_some_and(|s| !s.is_empty()),
            "Manifest requires a nonempty string: {name}"
        );
    }
    let mut report = compare(original, modified)?;
    let replacements: Vec<_> = report["files"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["changed_pixels"] != 0)
        .collect();
    ensure!(replacements.len() <= 2, "Package at most two assets");
    let mut outputs = Outputs::new();
    let mut names = BTreeSet::new();
    for row in replacements {
        let relative = Path::new(row["path"].as_str().unwrap());
        let name = relative.file_name().unwrap().to_str().unwrap();
        ensure!(
            name.starts_with("spr_"),
            "Expected the game's exact spr_ filename: {}",
            relative.display()
        );
        ensure!(
            names.insert(name.to_owned()),
            "Ambiguous replacement basename: {name}"
        );
        let meta: Metadata = toml::from_str(&fs::read_to_string(
            original.join(relative.with_extension("meta.toml")),
        )?)?;
        let Frames {
            frame_size: [w, h],
            frame_len,
        } = meta.asset_properties;
        let width = w.checked_mul(frame_len).context("Strip width overflow")?;
        ensure!(
            w > 0 && h > 0 && frame_len > 0 && row["size"] == json!([width, h]),
            "Expected an unchanged horizontal animation strip: {}",
            relative.display()
        );
        outputs.insert(
            format!("images/replace/{name}"),
            fs::read(modified.join(relative))?,
        );
    }
    outputs.insert("manifest.toml".into(), manifest);
    // MOMI dispatches other file extensions as mod instructions: keep JSON outside.
    write_tree(&output, outputs)?;
    report["replacements"] = json!(names);
    Ok(report)
}
