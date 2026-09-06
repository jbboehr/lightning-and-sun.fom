use crate::assets::*;
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

pub struct Variant {
    pub id: String,
    pub label: String,
    pub directory: PathBuf,
}

impl Variant {
    pub fn blue(directory: &Path) -> Self {
        Self {
            id: "blue".into(),
            label: "Debug Blue".into(),
            directory: directory.into(),
        }
    }
}

pub fn validate_variants(variants: &[Variant]) -> Result<()> {
    ensure!(
        (1..=8).contains(&variants.len()),
        "Select between one and eight presets"
    );
    let mut ids = BTreeSet::new();
    for variant in variants {
        ensure!(
            !variant.id.is_empty()
                && variant.id.len() <= 32
                && variant
                    .id
                    .bytes()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
                && variant.id != "vanilla"
                && ids.insert(&variant.id),
            "Preset IDs must be distinct lowercase names; vanilla is reserved"
        );
        ensure!(
            !variant.label.trim().is_empty()
                && variant.label.len() <= 64
                && !variant.label.chars().any(char::is_control),
            "Invalid preset label"
        );
    }
    Ok(())
}

pub fn variant_name(path: &str, id: &str) -> Result<String> {
    Ok(format!(
        "{}_{id}",
        sprite_pair(path)?[1].strip_suffix("_blue").unwrap()
    ))
}

const EXPRESSIONS: &str = "angry_blush blush cartoon_embarrassed concerned embarrassed embarrassed_tired evasive_tired gloomy_special happy happy_blush hope_special mad neutral neutral_tired sad shocked sick_eyes_closed sick_eyes_open sick_smile sick_think sigh sly think ugh wink";

pub fn sprite_pair(path: &str) -> Result<[String; 2]> {
    let source = Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .context("Invalid portrait filename")?;
    let expression = source
        .strip_prefix("spr_portrait_adeline_spring_")
        .context("Only Adeline spring portraits are supported")?;
    ensure!(
        EXPRESSIONS
            .split_whitespace()
            .any(|name| name == expression),
        "Unsupported Adeline spring expression: {expression}"
    );
    Ok([
        source.to_owned(),
        format!("spr_lns_adeline_spring_{expression}_blue"),
    ])
}

pub fn runtime_script(groups: &[Vec<String>], variants: &[Variant]) -> Result<Vec<u8>> {
    let labels: Vec<_> = std::iter::once("Vanilla")
        .chain(variants.iter().map(|v| v.label.as_str()))
        .collect();
    Ok(format!("// Generated from the portraits included in this local package.\nfunction lns_palette_assets() {{ return {}; }}\nfunction lns_palette_names() {{ return {}; }}\n", serde_json::to_string(groups)?, serde_json::to_string(&labels)?).into_bytes())
}

#[derive(Deserialize)]
struct Metadata {
    asset_properties: Frames,
}
#[derive(Deserialize)]
struct Frames {
    frame_size: [u32; 2],
    frame_len: u32,
    atlas: String,
}

pub fn package(original: &Path, modified: &Path, output: &Path) -> Result<Value> {
    package_variants(original, &[Variant::blue(modified)], output)
}

pub fn package_variants(original: &Path, variants: &[Variant], output: &Path) -> Result<Value> {
    validate_variants(variants)?;
    let inputs: Vec<_> = std::iter::once(original)
        .chain(variants.iter().map(|v| v.directory.as_path()))
        .collect();
    let output = fresh_output(output, &inputs)?;
    let reports = variants
        .iter()
        .map(|v| compare(original, &v.directory))
        .collect::<Result<Vec<_>>>()?;
    let mut report = reports[0].clone();
    let rows = report["files"].as_array().unwrap();
    ensure!(
        (1..=25).contains(&rows.len()),
        "Select between one and 25 Adeline spring portraits"
    );
    let mut names = BTreeSet::new();
    let mut pairs = Vec::new();
    let mut variant_paths = Vec::new();
    let mut outputs = Outputs::new();
    for row in rows {
        let relative = Path::new(row["path"].as_str().unwrap());
        let pair = sprite_pair(row["path"].as_str().unwrap())?;
        ensure!(
            names.insert(pair[0].clone()),
            "Duplicate portrait expression: {}",
            pair[0]
        );
        let text = fs::read_to_string(original.join(relative.with_extension("meta.toml")))?;
        let meta: Metadata = toml::from_str(&text)?;
        let Frames {
            frame_size: [w, h],
            frame_len,
            atlas,
        } = meta.asset_properties;
        let width = w.checked_mul(frame_len).context("Strip width overflow")?;
        ensure!(
            w > 0 && h > 0 && frame_len > 0 && row["size"] == json!([width, h]),
            "Expected an unchanged horizontal animation strip"
        );
        ensure!(
            atlas == "PortraitsSpring",
            "The toggle requires the PortraitsSpring atlas"
        );
        let mut meta: toml::Value = toml::from_str(&text)?;
        // MOMI assigns a fresh ID. Copying the source ID would replace vanilla.
        meta["meta_properties"] = toml::Value::try_from(std::collections::BTreeMap::from([(
            "asset_kind",
            "Animation",
        )]))?;
        let mut group = vec![pair[0].clone()];
        for variant in variants {
            let name = variant_name(row["path"].as_str().unwrap(), &variant.id)?;
            let destination = format!("animations/LightningAndSun/{name}");
            outputs.insert(
                format!("{destination}.png"),
                fs::read(variant.directory.join(relative))?,
            );
            outputs.insert(
                format!("{destination}.meta.toml"),
                toml::to_string_pretty(&meta)?.into_bytes(),
            );
            variant_paths.push(format!("{destination}.png"));
            group.push(name);
        }
        pairs.push(group);
    }
    outputs.insert(
        "manifest.toml".into(),
        include_bytes!("../mod/toggle/manifest.toml").to_vec(),
    );
    outputs.insert(
        "gml/palette_toggle.gml".into(),
        include_bytes!("../mod/toggle/gml/palette_toggle.gml").to_vec(),
    );
    outputs.insert(
        "gml/palette_assets.gml".into(),
        runtime_script(&pairs, variants)?,
    );
    write_tree(&output, outputs)?;
    if variants.len() > 1 {
        report = json!({
            "changed_pixels": reports.iter().map(|r| r["changed_pixels"].as_u64().unwrap()).sum::<u64>(),
            "presets": variants.iter().zip(&reports).map(|(v, r)| json!({"id":v.id,"label":v.label,"report":r})).collect::<Vec<_>>()
        });
    }
    report["variants"] = json!(variant_paths);
    report["preset_names"] = json!(variants.iter().map(|v| &v.label).collect::<Vec<_>>());
    report["hotkey"] = json!("F6");
    Ok(report)
}
