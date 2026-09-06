use crate::assets::*;
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::BTreeSet, fs, path::Path};

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

pub fn asset_script(pairs: &[[String; 2]]) -> Result<Vec<u8>> {
    Ok(format!("// Generated from the portraits included in this local package.\nfunction lns_palette_assets() {{ return {}; }}\n", serde_json::to_string(pairs)?).into_bytes())
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
    let output = fresh_output(output, &[original, modified])?;
    let mut report = compare(original, modified)?;
    let rows = report["files"].as_array().unwrap();
    ensure!(
        (1..=25).contains(&rows.len()),
        "Select between one and 25 Adeline spring portraits"
    );
    let mut names = BTreeSet::new();
    let mut pairs = Vec::new();
    let mut variants = Vec::new();
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
        let variant = format!("animations/LightningAndSun/{}", pair[1]);
        outputs.insert(format!("{variant}.png"), fs::read(modified.join(relative))?);
        outputs.insert(
            format!("{variant}.meta.toml"),
            toml::to_string_pretty(&meta)?.into_bytes(),
        );
        variants.push(format!("{variant}.png"));
        pairs.push(pair);
    }
    outputs.insert(
        "manifest.toml".into(),
        include_bytes!("../mod/toggle/manifest.toml").to_vec(),
    );
    outputs.insert(
        "gml/palette_toggle.gml".into(),
        include_bytes!("../mod/toggle/gml/palette_toggle.gml").to_vec(),
    );
    outputs.insert("gml/palette_assets.gml".into(), asset_script(&pairs)?);
    write_tree(&output, outputs)?;
    report["variants"] = json!(variants);
    report["hotkey"] = json!("F6");
    Ok(report)
}
