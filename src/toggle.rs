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
    pub atlas: &'static str,
    pub character: &'static crate::characters::Character,
    path: &'static str,
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
        self.path.to_owned()
    }
}
pub fn animation(path: &str) -> Result<Animation<'_>> {
    let source = Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .context("Invalid animation filename")?;
    for character in crate::characters::registry() {
        for (asset, atlas) in &character.animations {
            if Path::new(asset).file_stem().and_then(|s| s.to_str()) == Some(source) {
                return Ok(Animation {
                    source,
                    atlas,
                    character,
                    path: asset,
                });
            }
        }
    }
    anyhow::bail!("Unsupported animation: {source}")
}

pub struct PackageInput<'a> {
    pub original: &'a Path,
    pub variants: &'a [Variant],
}
pub struct RuntimeCharacter {
    pub character: &'static crate::characters::Character,
    pub groups: Vec<Vec<String>>,
    pub labels: Vec<String>,
}
impl RuntimeCharacter {
    pub fn new(groups: Vec<Vec<String>>, variants: &[Variant]) -> Result<Self> {
        let first = groups.first().context("Select at least one animation")?;
        let character = animation(&first[0])?.character;
        for group in &groups {
            ensure!(
                animation(&group[0])?.character.id == character.id,
                "Use a separate preset set for each character"
            );
        }
        let labels = std::iter::once("Vanilla".to_owned())
            .chain(variants.iter().map(|v| v.label.clone()))
            .collect();
        Ok(Self {
            character,
            groups,
            labels,
        })
    }
}
pub fn runtime_script(characters: &[RuntimeCharacter]) -> Result<Vec<u8>> {
    // Arrays are valid in both JSON and the game's GML dialect.
    let table: Vec<_> = characters
        .iter()
        .map(|c| {
            (
                &c.character.id,
                &c.character.label,
                &c.character.hotkey,
                &c.labels,
                &c.groups,
            )
        })
        .collect();
    Ok(format!("// Character rows: id, label, hotkey, preset names, animation groups.\nfunction lns_palette_definitions() {{ return {}; }}\n", serde_json::to_string(&table)?).into_bytes())
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
    package_characters(&[PackageInput { original, variants }], output)
}

pub fn package_characters(inputs: &[PackageInput<'_>], output: &Path) -> Result<Value> {
    ensure!(!inputs.is_empty(), "Select at least one character");
    let paths: Vec<_> = inputs
        .iter()
        .flat_map(|p| {
            std::iter::once(p.original).chain(p.variants.iter().map(|v| v.directory.as_path()))
        })
        .collect();
    let output = fresh_output(output, &paths)?;
    let mut outputs = Outputs::new();
    let mut runtime = Vec::new();
    let mut character_reports = Vec::new();
    let mut owners = BTreeSet::new();
    for input in inputs {
        let original = input.original;
        let variants = input.variants;
        validate_variants(variants)?;
        let reports = variants
            .iter()
            .map(|v| compare(original, &v.directory))
            .collect::<Result<Vec<_>>>()?;
        let mut report = reports[0].clone();
        let rows = report["files"].as_array().unwrap();
        ensure!(!rows.is_empty(), "Select at least one supported animation");
        let mut names = BTreeSet::new();
        let mut pairs = Vec::new();
        let mut variant_paths = Vec::new();
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
        let definition = RuntimeCharacter::new(pairs, variants)?;
        ensure!(
            owners.insert(&definition.character.id),
            "Duplicate character package"
        );
        runtime.push(definition);
        if variants.len() > 1 {
            report = json!({
                "changed_pixels": reports.iter().map(|r| r["changed_pixels"].as_u64().unwrap()).sum::<u64>(),
                "presets": variants.iter().zip(&reports).map(|(v, r)| json!({"id":v.id,"label":v.label,"report":r})).collect::<Vec<_>>()
            });
        }
        report["variants"] = json!(variant_paths);
        report["preset_names"] = json!(variants.iter().map(|v| &v.label).collect::<Vec<_>>());
        report["hotkey"] = json!(runtime.last().unwrap().character.hotkey);
        character_reports.push(report);
    }
    outputs.insert(
        "manifest.toml".into(),
        include_bytes!("../mod/toggle/manifest.toml").to_vec(),
    );
    outputs.insert(
        "gml/palette_toggle.gml".into(),
        include_bytes!("../mod/toggle/gml/palette_toggle.gml").to_vec(),
    );
    outputs.insert("gml/palette_assets.gml".into(), runtime_script(&runtime)?);
    write_tree(&output, outputs)?;
    if character_reports.len() == 1 {
        Ok(character_reports.remove(0))
    } else {
        Ok(json!({"characters":character_reports}))
    }
}
