use crate::{
    assets::*,
    commands,
    palette::{self, Palette, Profile},
    toggle::{self, Variant},
};
use anyhow::{Context, Result, ensure};
use rayon::prelude::*;
use serde::{Deserialize, Deserializer};
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
    #[serde(default, deserialize_with = "profile_path")]
    profile: Option<PathBuf>,
}
pub struct Set {
    pub variants: Vec<Variant>,
    pub palettes: Vec<Palette>,
}

fn profile_path<'de, D: Deserializer<'de>>(de: D) -> Result<Option<PathBuf>, D::Error> {
    PathBuf::deserialize(de).map(Some)
}

fn load_profile(path: &Path) -> Result<Profile> {
    let profile: Profile = serde_json::from_slice(&fs::read(path)?)?;
    ensure!(
        !profile.source_colors.is_empty()
            && profile.source_colors.len() <= 64
            && !profile.regions.is_empty(),
        "A profile needs 1–64 source colors and reviewed regions"
    );
    ensure!(
        profile.source_colors.iter().collect::<BTreeSet<_>>().len() == profile.source_colors.len(),
        "Duplicate profile source color"
    );
    Ok(profile)
}

pub fn load(path: &Path) -> Result<Set> {
    let definition: Definition = serde_json::from_slice(&fs::read(path)?)?;
    let parent = path.parent().unwrap_or(Path::new("."));
    let shared_profile = load_profile(&parent.join(definition.profile))?;
    let mut variants = Vec::new();
    let mut palettes: Vec<Palette> = Vec::new();
    for preset in definition.presets {
        let override_profile = preset
            .profile
            .map(|profile| load_profile(&parent.join(profile)))
            .transpose()?;
        let profile = override_profile.as_ref().unwrap_or(&shared_profile);
        ensure!(
            preset.colors.len() == profile.source_colors.len(),
            "Preset ramp length differs from profile: {}",
            preset.id
        );
        let mapping: BTreeMap<_, _> = profile.source_colors.iter().zip(&preset.colors).collect();
        let p = palette::parse(&serde_json::to_vec(
            &json!({"rgba_map":mapping,"color_groups":profile.color_groups,"regions":profile.regions}),
        )?)?;
        if let Some(first) = palettes.first() {
            ensure!(
                p.assets() == first.assets(),
                "Preset profiles must select the same asset inventory: {}",
                preset.id
            );
        }
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
    /// Identifies the recolored output for any given source images.
    pub fn fingerprint(&self) -> String {
        let ids: Vec<_> = self.variants.iter().map(|v| &v.id).collect();
        digest(format!("{ids:?}{:?}", self.palettes).as_bytes())
    }
    /// Reuses finished variants already under `root`, so callers must give
    /// each recipe set its own root; see `fingerprint`.
    pub fn generate(
        &mut self,
        original: &Path,
        root: &Path,
        allow_source_hash_mismatch: bool,
        variant_done: &(dyn Fn(bool) + Sync),
    ) -> Result<Vec<Value>> {
        self.variants
            .par_iter_mut()
            .zip(&self.palettes)
            .map(|(variant, palette)| {
                variant.directory = root.join(&variant.id);
                let finished = finished(
                    &variant.directory,
                    original,
                    palette,
                    allow_source_hash_mismatch,
                )?;
                let reused = finished.is_some();
                let report = match finished {
                    Some(report) => report,
                    None => {
                        // Generate beside the final name so an interrupted run never looks finished.
                        let partial = root.join(format!("{}.partial", variant.id));
                        for stale in [&variant.directory, &partial] {
                            if stale.exists() {
                                fs::remove_dir_all(stale)?;
                            }
                        }
                        let report = commands::apply_palette(
                            original,
                            palette,
                            &partial,
                            allow_source_hash_mismatch,
                        )?;
                        fs::rename(&partial, &variant.directory)?;
                        report
                    }
                };
                variant_done(reused);
                Ok(json!({"id":variant.id,"label":variant.label,"report":report}))
            })
            .collect()
    }
}

/// The report of a variant left by an earlier run, if every file still matches it.
fn finished(
    directory: &Path,
    original: &Path,
    palette: &Palette,
    allow_source_hash_mismatch: bool,
) -> Result<Option<Value>> {
    let Ok(report) = fs::read(directory.join("palette-report.json")) else {
        return Ok(None);
    };
    let Ok(report) = serde_json::from_slice::<Value>(&report) else {
        return Ok(None);
    };
    let Ok((made_images, made_metadata)) = inventory(directory) else {
        return Ok(None);
    };
    let (images, metadata) = inventory(original)?;
    let rows = report["files"].as_array().map_or(&[][..], Vec::as_slice);
    if !(images.keys().eq(made_images.keys())
        && metadata.keys().eq(made_metadata.keys())
        && rows.len() == images.len())
    {
        return Ok(None);
    }
    for (row, (name, path)) in rows.iter().zip(&images) {
        let before = fs::read(path)?;
        // A fresh run would warn about, or refuse, a changed source here.
        palette.check_source(name, &before, allow_source_hash_mismatch)?;
        if row["path"] != name.as_str()
            || row["original_sha256"] != digest(&before)
            || row["modified_sha256"] != file_digest(&made_images[name])?
        {
            return Ok(None);
        }
    }
    for (name, path) in &metadata {
        if fs::read(path)? != fs::read(&made_metadata[name])? {
            return Ok(None);
        }
    }
    Ok(Some(report))
}

pub fn build(
    original: &Path,
    definition: &Path,
    output: &Path,
    allow_source_hash_mismatch: bool,
) -> Result<Value> {
    let output = fresh_output(output, &[original, definition])?;
    let mut set = load(definition)?;
    let work = tempfile::tempdir()?;
    let reports = set.generate(
        original,
        &work.path().join("variants"),
        allow_source_hash_mismatch,
        &|_| (),
    )?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};
    use std::sync::atomic::{AtomicBool, Ordering};

    fn write_original(original: &Path, color: [u8; 4]) {
        let strip = DynamicImage::ImageRgba8(RgbaImage::from_pixel(4, 1, Rgba(color)));
        fs::create_dir_all(original.join("assets")).unwrap();
        fs::write(original.join("assets/a.png"), png(&strip).unwrap()).unwrap();
        fs::write(original.join("assets/a.meta.toml"), "x = 1\n").unwrap();
    }

    fn set(target: &str) -> Set {
        Set {
            variants: vec![Variant {
                id: "blue".into(),
                label: "Blue".into(),
                directory: PathBuf::new(),
            }],
            palettes: vec![
                palette::parse(format!(r##"{{"rgba_map":{{"#E3A17B":"{target}"}}}}"##).as_bytes())
                    .unwrap(),
            ],
        }
    }

    fn output(set: &Set) -> RgbaImage {
        rgba(&fs::read(set.variants[0].directory.join("assets/a.png")).unwrap()).unwrap()
    }

    #[test]
    fn rerun_reuses_finished_variants_and_redoes_damaged_or_partial_ones() {
        let temp = tempfile::tempdir().unwrap();
        let (original, root) = (temp.path().join("original"), temp.path().join("variants"));
        write_original(&original, [0xE3, 0xA1, 0x7B, 255]);
        let mut blue = set("#9DB9D4");
        let reused = AtomicBool::new(false);
        let record = |was_reused| reused.store(was_reused, Ordering::Relaxed);
        let first = blue.generate(&original, &root, false, &record).unwrap();
        assert!(!reused.load(Ordering::Relaxed));
        assert_eq!(
            blue.generate(&original, &root, false, &record).unwrap(),
            first
        );
        assert!(reused.load(Ordering::Relaxed));

        let finished = blue.variants[0].directory.clone();
        fs::write(finished.join("assets/a.png"), b"damaged").unwrap();
        assert_eq!(
            blue.generate(&original, &root, false, &record).unwrap(),
            first
        );
        assert!(!reused.load(Ordering::Relaxed));
        assert_eq!(output(&blue).get_pixel(0, 0).0, [0x9D, 0xB9, 0xD4, 255]);

        fs::rename(&finished, finished.with_extension("partial")).unwrap();
        assert_eq!(
            blue.generate(&original, &root, false, &record).unwrap(),
            first
        );
        assert!(!reused.load(Ordering::Relaxed));
        assert!(!finished.with_extension("partial").exists());
    }

    #[test]
    fn rerun_redoes_variants_whose_source_changed() {
        let temp = tempfile::tempdir().unwrap();
        let (original, root) = (temp.path().join("original"), temp.path().join("variants"));
        write_original(&original, [0xE3, 0xA1, 0x7B, 255]);
        let mut blue = set("#9DB9D4");
        blue.generate(&original, &root, false, &|_| ()).unwrap();

        fs::remove_dir_all(&original).unwrap();
        write_original(&original, [1, 2, 3, 255]);
        blue.generate(&original, &root, false, &|_| ()).unwrap();
        assert_eq!(output(&blue).get_pixel(0, 0).0, [1, 2, 3, 255]);
    }

    #[test]
    fn fingerprint_changes_with_the_recipe() {
        assert_eq!(set("#9DB9D4").fingerprint(), set("#9DB9D4").fingerprint());
        assert_ne!(set("#9DB9D4").fingerprint(), set("#00FF00").fingerprint());
    }
}
