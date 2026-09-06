use crate::{
    assets::*,
    commands,
    palette::{self, Palette, Profile},
    toggle::{self, Variant},
};
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Definition {
    profile: PathBuf,
    presets: Vec<Preset>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Preset {
    id: String,
    label: String,
    colors: Vec<String>,
}
pub struct Set {
    pub variants: Vec<Variant>,
    pub palettes: Vec<Palette>,
}

pub fn load(path: &Path) -> Result<Set> {
    let definition: Definition = serde_json::from_slice(&fs::read(path)?)?;
    let profile: Profile = serde_json::from_slice(&fs::read(
        path.parent()
            .unwrap_or(Path::new("."))
            .join(definition.profile),
    )?)?;
    ensure!(
        !profile.source_colors.is_empty()
            && profile.source_colors.len() <= 32
            && !profile.regions.is_empty(),
        "A profile needs source colors and reviewed regions"
    );
    ensure!(
        profile.source_colors.iter().collect::<BTreeSet<_>>().len() == profile.source_colors.len(),
        "Duplicate profile source color"
    );
    let mut variants = Vec::new();
    let mut palettes = Vec::new();
    for preset in definition.presets {
        ensure!(
            preset.colors.len() == profile.source_colors.len(),
            "Preset ramp length differs from profile: {}",
            preset.id
        );
        let mapping: BTreeMap<_, _> = profile.source_colors.iter().zip(&preset.colors).collect();
        let p = palette::parse(&serde_json::to_vec(
            &json!({"rgba_map":mapping,"regions":profile.regions}),
        )?)?;
        palettes.push(p);
        variants.push(Variant {
            id: preset.id,
            label: preset.label,
            directory: PathBuf::new(),
        });
    }
    toggle::validate_variants(&variants)?;
    Ok(Set { variants, palettes })
}

impl Set {
    pub fn assets(&self) -> Vec<String> {
        self.palettes[0].assets().unwrap()
    }
    pub fn generate(&mut self, original: &Path, root: &Path) -> Result<Vec<Value>> {
        let mut reports = Vec::new();
        for (variant, palette) in self.variants.iter_mut().zip(&self.palettes) {
            variant.directory = root.join(&variant.id);
            let report = commands::apply_palette(original, palette, &variant.directory)?;
            reports.push(json!({"id":variant.id,"label":variant.label,"report":report}));
        }
        Ok(reports)
    }
}

pub fn build(original: &Path, definition: &Path, output: &Path) -> Result<Value> {
    let output = fresh_output(output, &[original, definition])?;
    let mut set = load(definition)?;
    let work = tempfile::tempdir()?;
    let reports = set.generate(original, &work.path().join("variants"))?;
    let package = toggle::package_variants(original, &set.variants, &work.path().join("package"))?;
    let report = json!({"presets":reports,"package":package});
    let mut outputs = Outputs::new();
    for entry in walkdir::WalkDir::new(work.path()) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let name = entry
                .path()
                .strip_prefix(work.path())?
                .to_str()
                .context("Non UTF-8 generated path")?
                .to_owned();
            outputs.insert(name, fs::read(entry.path())?);
        }
    }
    outputs.insert("presets-report.json".into(), json_bytes(&report)?);
    write_tree(&output, outputs)?;
    Ok(report)
}
