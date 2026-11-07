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
        (1..=11).contains(&variants.len()),
        "Select between one and eleven presets"
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
    pub ids: Vec<String>,
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
        let ids = std::iter::once("vanilla".to_owned())
            .chain(variants.iter().map(|v| v.id.clone()))
            .collect();
        Ok(Self {
            character,
            groups,
            labels,
            ids,
        })
    }
}

fn runtime_constant_bound(characters: &[RuntimeCharacter]) -> usize {
    let strings: usize = characters
        .iter()
        .map(|c| 3 + c.labels.len() + c.ids.len() + c.groups.iter().map(Vec::len).sum::<usize>())
        .sum();
    let indices = characters
        .iter()
        .flat_map(|c| {
            [6, c.labels.len(), c.ids.len(), c.groups.len()]
                .into_iter()
                .chain(c.groups.iter().map(Vec::len))
        })
        .chain([characters.len()])
        .max()
        .unwrap_or(0);
    // Fabricator pools constants per function, with u16 indexes. String
    // occurrences overestimate distinct strings; array indexes span 0..max_len.
    strings + indices
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
                &c.ids,
            )
        })
        .collect();
    let mut script =
        "// Character rows: id, label, hotkey, preset names, animation groups, preset ids.\n"
            .to_owned();
    let constant_limit = usize::from(u16::MAX) + 1;
    if runtime_constant_bound(characters) <= constant_limit {
        script.push_str(&format!(
            "function lns_palette_definitions() {{ return {}; }}\n",
            serde_json::to_string(&table)?
        ));
    } else {
        // Keep the runtime's ordered six-field rows intact while giving each
        // character its own constant pool. The dispatcher needs at most one
        // helper name and one array index per row.
        ensure!(
            characters.len() <= constant_limit / 2,
            "Too many character definitions"
        );
        let mut calls = Vec::new();
        for (index, (row, character)) in table.iter().zip(characters).enumerate() {
            ensure!(
                runtime_constant_bound(std::slice::from_ref(character)) <= constant_limit,
                "{} palette definition exceeds the native compiler constant limit",
                character.character.label
            );
            script.push_str(&format!(
                "function __lns_palette_definition_{index}() {{ return {}; }}\n",
                serde_json::to_string(row)?
            ));
            calls.push(format!("__lns_palette_definition_{index}()"));
        }
        script.push_str(&format!(
            "function lns_palette_definitions() {{ return [{}]; }}\n",
            calls.join(",")
        ));
    }
    Ok(script.into_bytes())
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
    outputs.insert(
        "gml/palette_controls.gml".into(),
        include_bytes!("../mod/toggle/gml/palette_controls.gml").to_vec(),
    );
    write_tree(&output, outputs)?;
    if character_reports.len() == 1 {
        Ok(character_reports.remove(0))
    } else {
        Ok(json!({"characters":character_reports}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn full_runtime() -> Vec<RuntimeCharacter> {
        crate::characters::Collection::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("palettes/sets/characters-trial.json"),
        )
        .unwrap()
        .selected
        .into_iter()
        .map(|selected| {
            let groups = selected
                .set
                .assets()
                .iter()
                .map(|path| {
                    let animation = animation(path).unwrap();
                    std::iter::once(animation.source.to_owned())
                        .chain(
                            selected
                                .set
                                .variants
                                .iter()
                                .map(|variant| animation.variant_name(&variant.id)),
                        )
                        .collect()
                })
                .collect();
            RuntimeCharacter::new(groups, &selected.set.variants).unwrap()
        })
        .collect()
    }

    fn row(character: &RuntimeCharacter) -> Value {
        json!([
            character.character.id,
            character.character.label,
            character.character.hotkey,
            character.labels,
            character.groups,
            character.ids,
        ])
    }

    #[test]
    fn small_runtime_keeps_the_flat_json_byte_contract() {
        let runtime = RuntimeCharacter {
            character: crate::characters::get("adeline").unwrap(),
            groups: vec![vec!["original".into(), "blue".into()]],
            labels: vec!["Vanilla".into(), "Debug Blue".into()],
            ids: vec!["vanilla".into(), "blue".into()],
        };
        let expected = format!(
            "// Character rows: id, label, hotkey, preset names, animation groups, preset ids.\nfunction lns_palette_definitions() {{ return [{}]; }}\n",
            row(&runtime)
        );
        assert_eq!(runtime_script(&[runtime]).unwrap(), expected.as_bytes());
    }

    #[test]
    fn full_native_tone_runtime_splits_literals_without_changing_rows() {
        let runtime = full_runtime();
        assert_eq!(runtime.len(), 36);
        for character in &runtime {
            assert_eq!(character.ids.len(), 12);
            assert_eq!(character.ids.last().unwrap(), "player_27");
            assert_eq!(character.labels.len(), 12);
            assert_eq!(character.labels.last().unwrap(), "Deep umber (Player 27)");
            assert!(character.groups.iter().all(|group| group.len() == 12));
        }
        assert_eq!(runtime.iter().map(|c| c.groups.len()).sum::<usize>(), 6312);
        let script = String::from_utf8(runtime_script(&runtime).unwrap()).unwrap();
        let functions: Vec<_> = script.lines().skip(1).collect();
        assert_eq!(
            functions.len(),
            runtime.len() + 1,
            "large table must use one literal helper per character"
        );
        let mut calls = Vec::new();
        for (index, (definition, character)) in functions.iter().zip(&runtime).enumerate() {
            let prefix = format!("function __lns_palette_definition_{index}() {{ return ");
            let literal = definition
                .strip_prefix(&prefix)
                .unwrap()
                .strip_suffix("; }")
                .unwrap();
            let emitted: Value = serde_json::from_str(literal).unwrap();
            assert_eq!(
                emitted,
                row(character),
                "{} row changed",
                character.character.id
            );
            let strings = 3
                + character.labels.len()
                + character.ids.len()
                + character.groups.iter().map(Vec::len).sum::<usize>();
            let indices = [
                6,
                character.labels.len(),
                character.ids.len(),
                character.groups.len(),
            ]
            .into_iter()
            .chain(character.groups.iter().map(Vec::len))
            .max()
            .unwrap();
            assert!(strings + indices <= usize::from(u16::MAX) + 1);
            calls.push(format!("__lns_palette_definition_{index}()"));
        }
        assert_eq!(
            functions.last().unwrap(),
            &format!(
                "function lns_palette_definitions() {{ return [{}]; }}",
                calls.join(",")
            )
        );
        assert!(runtime.len() * 2 <= usize::from(u16::MAX) + 1);
    }

    #[test]
    #[ignore = "requires a Fabricator interpreter in FOM_GML_INTERPRETER"]
    fn emitted_full_native_tone_table_compiles_and_evaluates_exact_rows() {
        let runtime = full_runtime();
        let mut script = String::from_utf8(runtime_script(&runtime).unwrap()).unwrap();
        script.push_str(
            r#"function assert_row(actual, expected) {
assert(array_length(actual) == 6);
for (var i = 0; i < 3; i++) assert(actual[i] == expected[i]);
for (var i = 3; i < 6; i++) {
assert(array_length(actual[i]) == array_length(expected[i]));
for (var j = 0; j < array_length(expected[i]); j++) {
if (i == 4) {
assert(array_length(actual[i][j]) == array_length(expected[i][j]));
for (var k = 0; k < array_length(expected[i][j]); k++) assert(actual[i][j][k] == expected[i][j][k]);
} else assert(actual[i][j] == expected[i][j]);
}
}
}
"#,
        );
        for (index, character) in runtime.iter().enumerate() {
            script.push_str(&format!(
                "function test_row_{index}(actual) {{ assert_row(actual, {}); }}\n",
                row(character)
            ));
        }
        script.push_str(
            "var table = lns_palette_definitions();\nassert(array_length(table) == 36);\n",
        );
        for index in 0..runtime.len() {
            script.push_str(&format!("test_row_{index}(table[{index}]);\n"));
        }
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("native_tone_table.gml");
        fs::write(&path, script).unwrap();
        let result =
            Command::new(std::env::var_os("FOM_GML_INTERPRETER").expect("Set FOM_GML_INTERPRETER"))
                .arg("run")
                .arg(path)
                .output()
                .unwrap();
        assert!(
            result.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
    }
}
