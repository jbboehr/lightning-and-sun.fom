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

pub struct Animation<'a> {
    pub source: &'a str,
    pub season: &'static str,
    pub atlas: &'static str,
    pub category: &'static str,
}

impl Animation<'_> {
    pub fn variant_name(&self, id: &str) -> String {
        format!(
            "spr_lns_{}_{id}",
            self.source
                .strip_prefix("spr_portrait_")
                .or_else(|| self.source.strip_prefix("spr_"))
                .unwrap()
        )
    }

    pub fn asset_path(&self) -> String {
        format!(
            "assets/animations/NPCs/Adeline/{}/{}/{}.png",
            self.category, self.season, self.source
        )
    }
}

const EXPRESSIONS: &str = "angry_blush blush cartoon_embarrassed concerned embarrassed embarrassed_tired evasive_tired gloomy_special happy happy_blush hope_special mad neutral neutral_tired sad shocked sick_eyes_closed sick_eyes_open sick_smile sick_think sigh sly think ugh wink";
const BEACH_EXPRESSIONS: &str = "angry_blush bath_neutral blush cartoon_embarrassed concerned embarrassed gloomy_special happy happy_blush hope_special mad neutral sad shocked sigh sly think ugh wink";
const WEDDING_EXPRESSIONS: &str = "embarrassed happy_blush hope_special neutral sad sly think";

pub fn animation(path: &str) -> Result<Animation<'_>> {
    let source = Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .context("Invalid animation filename")?;
    if let Some(cycle) = source.strip_prefix("spr_npc_adeline_spring_") {
        ensure!(
            "idle_north idle_south idle_east walk_north walk_south walk_east sit_north sit_south sit_east drink_north drink_south drink_east eat_north eat_south eat_east blink_south blink_east"
                .split_whitespace()
                .any(|name| name == cycle),
            "Unsupported Adeline spring world animation: {cycle}"
        );
        return Ok(Animation {
            source,
            season: "Spring",
            atlas: "Default",
            category: "Sprites",
        });
    }
    let (season, atlas, expression) =
        if let Some(expression) = source.strip_prefix("spr_portrait_adeline_spring_") {
            ("Spring", "PortraitsSpring", expression)
        } else if let Some(expression) = source.strip_prefix("spr_portrait_adeline_summer_") {
            ("Summer", "PortraitsSummer", expression)
        } else if let Some(expression) = source.strip_prefix("spr_portrait_adeline_autumn_") {
            ("Autumn", "PortraitsAutumn", expression)
        } else if let Some(expression) = source.strip_prefix("spr_portrait_adeline_winter_") {
            ("Winter", "PortraitsWinter", expression)
        } else if let Some(expression) = source.strip_prefix("spr_portrait_adeline_beach_") {
            ("Beach", "PortraitsSummer", expression)
        } else if let Some(expression) = source.strip_prefix("spr_portrait_adeline_wedding_") {
            ("Wedding", "PortraitsMisc", expression)
        } else {
            anyhow::bail!("Only Adeline portraits and reviewed spring world sprites are supported");
        };
    let expressions = match season {
        "Beach" => BEACH_EXPRESSIONS,
        "Wedding" => WEDDING_EXPRESSIONS,
        _ => EXPRESSIONS,
    };
    ensure!(
        expressions
            .split_whitespace()
            .any(|name| name == expression),
        "Unsupported Adeline {season} expression: {expression}"
    );
    Ok(Animation {
        source,
        season,
        atlas,
        category: "Portraits",
    })
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
    #[serde(default = "default_frame_len")]
    frame_len: u32,
    atlas: String,
}

fn default_frame_len() -> u32 {
    1
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
        (1..=143).contains(&rows.len()),
        "Select between one and 143 supported Adeline animations"
    );
    let mut names = BTreeSet::new();
    let mut pairs = Vec::new();
    let mut variant_paths = Vec::new();
    let mut outputs = Outputs::new();
    for row in rows {
        let relative = Path::new(row["path"].as_str().unwrap());
        let animation = animation(row["path"].as_str().unwrap())?;
        ensure!(
            names.insert(animation.source),
            "Duplicate animation expression: {}",
            animation.source
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
            atlas == animation.atlas,
            "The animation requires the {} atlas",
            animation.atlas
        );
        let mut meta: toml::Value = toml::from_str(&text)?;
        // MOMI assigns a fresh ID. Copying the source ID would replace vanilla.
        meta["meta_properties"] = toml::Value::try_from(std::collections::BTreeMap::from([(
            "asset_kind",
            "Animation",
        )]))?;
        let mut group = vec![animation.source.to_owned()];
        for variant in variants {
            let name = animation.variant_name(&variant.id);
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
