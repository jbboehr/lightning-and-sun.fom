use crate::assets::*;
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{fs, path::Path};

const SOURCE: &str = "spr_portrait_adeline_spring_neutral.png";
const VARIANT: &str = "animations/LightningAndSun/spr_lns_adeline_spring_neutral_blue";

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
    let output = fresh_output(output, &[original, modified])?;
    let mut report = compare(original, modified)?;
    let rows = report["files"].as_array().unwrap();
    ensure!(
        rows.len() == 1,
        "The toggle study requires exactly one portrait"
    );
    let row = &rows[0];
    let relative = Path::new(row["path"].as_str().unwrap());
    ensure!(
        relative.file_name().is_some_and(|name| name == SOURCE),
        "The toggle study only supports {SOURCE}"
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
        "The toggle study requires the PortraitsSpring atlas"
    );
    let mut meta: toml::Value = toml::from_str(&text)?;
    // MOMI assigns a fresh ID. Copying the source ID would replace vanilla.
    meta["meta_properties"] = toml::Value::try_from(std::collections::BTreeMap::from([(
        "asset_kind",
        "Animation",
    )]))?;
    let outputs = Outputs::from([
        (format!("{VARIANT}.png"), fs::read(modified.join(relative))?),
        (
            format!("{VARIANT}.meta.toml"),
            toml::to_string_pretty(&meta)?.into_bytes(),
        ),
        (
            "manifest.toml".into(),
            include_bytes!("../mod/toggle/manifest.toml").to_vec(),
        ),
        (
            "gml/palette_toggle.gml".into(),
            include_bytes!("../mod/toggle/gml/palette_toggle.gml").to_vec(),
        ),
    ]);
    write_tree(&output, outputs)?;
    report["variant"] = json!(format!("{VARIANT}.png"));
    report["hotkey"] = json!("F6");
    Ok(report)
}
