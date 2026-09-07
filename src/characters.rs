use crate::{assets::*, commands, presets, toggle};
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Character {
    pub id: String,
    pub label: String,
    pub hotkey: String,
    pub animations: BTreeMap<String, String>,
}
pub fn registry() -> &'static [Character] {
    static REGISTRY: OnceLock<Vec<Character>> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        serde_json::from_slice(include_bytes!("../mod/config/characters.json"))
            .expect("checked character registry")
    })
}
pub fn get(id: &str) -> Result<&'static Character> {
    registry()
        .iter()
        .find(|c| c.id == id)
        .with_context(|| format!("Unsupported character: {id}"))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Definition {
    characters: Vec<Selection>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Selection {
    id: String,
    presets: PathBuf,
}
pub struct Selected {
    pub character: &'static Character,
    pub set: presets::Set,
    pub original: PathBuf,
}
pub struct Collection {
    pub selected: Vec<Selected>,
}
impl Collection {
    pub fn load(path: &Path) -> Result<Self> {
        let definition: Definition = serde_json::from_slice(&fs::read(path)?)?;
        ensure!(
            !definition.characters.is_empty(),
            "Select at least one character"
        );
        let mut ids = BTreeSet::new();
        let mut selected = Vec::new();
        for selection in definition.characters {
            let character = get(&selection.id)?;
            ensure!(ids.insert(&character.id), "Duplicate character selection");
            let set = presets::load(
                &path
                    .parent()
                    .unwrap_or(Path::new("."))
                    .join(selection.presets),
            )?;
            for asset in set.assets() {
                ensure!(
                    character.animations.contains_key(&asset),
                    "Expected an exact reviewed {} animation path: {asset}",
                    character.label
                );
            }
            selected.push(Selected {
                character,
                set,
                original: PathBuf::new(),
            });
        }
        Ok(Self { selected })
    }
    pub fn generate(&mut self, archive: &Path, root: &Path) -> Result<Value> {
        let mut reports = Vec::new();
        for selected in &mut self.selected {
            let directory = root.join(&selected.character.id);
            selected.original = directory.join("original");
            let assets = selected.set.assets();
            eprintln!(
                "Generating {} animations for {} from local game assets...",
                assets.len(),
                selected.character.label
            );
            commands::export(archive, &assets, &selected.original)?;
            let presets = selected
                .set
                .generate(&selected.original, &directory.join("variants"))?;
            reports.push(json!({"id":selected.character.id,"hotkey":selected.character.hotkey,"presets":presets}));
        }
        Ok(json!({"characters":reports}))
    }
    pub fn inputs(&self) -> Vec<toggle::PackageInput<'_>> {
        self.selected
            .iter()
            .map(|c| toggle::PackageInput {
                original: &c.original,
                variants: &c.set.variants,
            })
            .collect()
    }
}
pub fn build(archive: &Path, definition: &Path, output: &Path) -> Result<Value> {
    let output = fresh_output(output, &[archive, definition])?;
    let mut collection = Collection::load(definition)?;
    let work = tempfile::tempdir()?;
    let mut report = collection.generate(archive, &work.path().join("characters"))?;
    report["package"] =
        toggle::package_characters(&collection.inputs(), &work.path().join("package"))?;
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
    write_tree(&output, outputs)?;
    fs::write(
        output.join("characters-report.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn registry_has_unique_owners_assets_and_controls() {
        let mut ids = BTreeSet::new();
        let mut hotkeys = BTreeSet::new();
        let mut sources = BTreeSet::new();
        for character in registry() {
            assert!(ids.insert(&character.id));
            assert!(hotkeys.insert(&character.hotkey));
            assert!(
                !character.id.is_empty()
                    && character
                        .id
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b == b'_')
            );
            assert!(!character.animations.is_empty());
            for path in character.animations.keys() {
                let source = Path::new(path).file_stem().unwrap().to_str().unwrap();
                assert!(sources.insert(source));
                assert!(path.starts_with(&format!("assets/animations/NPCs/{}/", character.label)));
                assert!(
                    source.starts_with(&format!("spr_portrait_{}_", character.id))
                        || source.starts_with(&format!("spr_npc_{}_", character.id))
                );
            }
        }
    }
}
