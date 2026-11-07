use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
fn rgba(text: &str) -> [u8; 4] {
    let mut result = [255; 4];
    for (i, channel) in result.iter_mut().enumerate().take((text.len() - 1) / 2) {
        *channel = u8::from_str_radix(&text[1 + i * 2..3 + i * 2], 16).unwrap();
    }
    result
}
fn blush_trial() -> (Value, Value) {
    let set = read(root().join("palettes/sets/celine-native-blush-trial.json"));
    let profile = read(
        root()
            .join("palettes/sets")
            .join(set["profile"].as_str().unwrap()),
    );
    (set, profile)
}

#[test]
fn celine_blush_profile_only_appends_isolated_source_bound_cheek_components() {
    let (set, profile) = blush_trial();
    let base = read(root().join("palettes/profiles/celine-world-trial.json"));
    let historical = read(root().join("palettes/sets/celine-world-trial.json"));
    let colors = json!(["#F4AEAE", "#FCB0B6", "#E6687A"]);
    assert_eq!(set["profile"], "../profiles/celine-native-tone-blush.json");
    assert_eq!(
        &profile["source_colors"].as_array().unwrap()[..14],
        base["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &profile["source_colors"].as_array().unwrap()[14..],
        colors.as_array().unwrap()
    );
    assert_eq!(
        &profile["color_groups"].as_array().unwrap()[..7],
        base["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        &profile["color_groups"].as_array().unwrap()[7..],
        &[json!(["#F4AEAE"]), json!(["#FCB0B6"]), json!(["#E6687A"])]
    );
    assert_eq!(profile["regions"].as_array().unwrap().len(), 390);
    let (mut corrected, mut seeds) = (0, 0);
    for (region, old) in profile["regions"]
        .as_array()
        .unwrap()
        .iter()
        .zip(base["regions"].as_array().unwrap())
    {
        for key in ["asset", "source_sha256", "size"] {
            assert_eq!(region[key], old[key]);
        }
        let old_seeds = old["seeds"].as_array().unwrap();
        let new_seeds = region["seeds"].as_array().unwrap();
        assert_eq!(&new_seeds[..old_seeds.len()], old_seeds);
        if new_seeds.len() > old_seeds.len() {
            assert!(old["asset"].as_str().unwrap().contains("/Portraits/"));
            corrected += 1;
            for seed in &new_seeds[old_seeds.len()..] {
                let x = seed[0].as_u64().unwrap() % 296;
                let y = seed[1].as_u64().unwrap();
                assert!((146..=157).contains(&x) && (75..=82).contains(&y));
                seeds += 1;
            }
        }
    }
    assert_eq!((corrected, seeds), (63, 334));
    for (i, preset) in set["presets"].as_array().unwrap().iter().enumerate() {
        let actual = &preset["colors"].as_array().unwrap()[14..];
        if i < 4 {
            let old = &historical["presets"][i];
            assert_eq!(preset["id"], old["id"]);
            assert_eq!(preset["label"], old["label"]);
            assert_eq!(
                &preset["colors"].as_array().unwrap()[..14],
                old["colors"].as_array().unwrap()
            );
            assert_eq!(actual, colors.as_array().unwrap());
        } else {
            let donor = rgba(preset["colors"][0].as_str().unwrap());
            for (source, target) in colors.as_array().unwrap().iter().zip(actual) {
                let original = rgba(source.as_str().unwrap());
                let expected: Vec<_> = (0..3)
                    .map(|channel| {
                        let n = u32::from(original[channel]) * u32::from(donor[channel]);
                        let d = u32::from([252u8, 222, 190][channel]);
                        ((2 * n + d) / (2 * d)) as u8
                    })
                    .collect();
                assert_eq!(&rgba(target.as_str().unwrap())[..3], expected);
            }
        }
    }
}

#[test]
#[ignore = "requires the retained source-bound Celine portrait corpus"]
fn reported_celine_cheeks_follow_deep_tone_while_lips_keep_their_source_colors() {
    let (set, mut profile) = blush_trial();
    let source = root().join("generated/native-tone-build/characters/celine/original");
    let temp = tempfile::tempdir().unwrap();
    let names = ["angry_blush", "blush", "blush_special", "neutral"];
    let assets:Vec<_>=names.iter().map(|name|format!("assets/animations/NPCs/Celine/Portraits/Autumn/Garden/spr_portrait_celine_autumn_garden_{name}.png")).collect();
    profile["regions"]
        .as_array_mut()
        .unwrap()
        .retain(|r| assets.iter().any(|a| r["asset"] == *a));
    let input = temp.path().join("original");
    for asset in &assets {
        let target = input.join(asset);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        for p in [
            PathBuf::from(asset),
            PathBuf::from(asset.replace(".png", ".meta.toml")),
        ] {
            fs::copy(source.join(&p), input.join(p)).unwrap();
        }
    }
    let profile_path = temp.path().join("profile.json");
    fs::write(&profile_path, serde_json::to_vec(&profile).unwrap()).unwrap();
    let preset = set["presets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == "player_35")
        .unwrap();
    let map: serde_json::Map<_, _> = profile["source_colors"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, c)| (c.as_str().unwrap().to_owned(), preset["colors"][i].clone()))
        .collect();
    let recipe = temp.path().join("recipe.json");
    fs::write(
        &recipe,
        serde_json::to_vec(&json!({"profile":profile_path,"rgba_map":map})).unwrap(),
    )
    .unwrap();
    let output = temp.path().join("output");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("apply")
        .arg("--input")
        .arg(&input)
        .arg("--palette")
        .arg(recipe)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for (index, asset) in assets.iter().enumerate() {
        let before = image::open(input.join(asset)).unwrap().to_rgba8();
        let after = image::open(output.join(asset)).unwrap().to_rgba8();
        for frame in 0..2 {
            let dx = frame * 296;
            let points: Vec<(u32, u32, &str, &str)> = match index {
                0 => vec![(153, 77, "#F4AEAE", "#48121A")],
                1 => vec![(152, 76, "#F4AEAE", "#48121A")],
                2 => vec![
                    (150, 79, "#FCB0B6", "#4A121B"),
                    (148, 80, "#E6687A", "#440B12"),
                ],
                _ if frame == 0 => vec![
                    (151, 85, "#FCB0B6", "#FCB0B6"),
                    (152, 84, "#E6687A", "#E6687A"),
                ],
                _ => vec![(151, 85, "#A5462C", "#A5462C")],
            };
            for (x, y, original, expected) in points {
                assert_eq!(
                    before.get_pixel(dx + x, y).0,
                    rgba(original),
                    "source landmark {asset} [{x},{y}] frame {frame}"
                );
                assert_eq!(
                    after.get_pixel(dx + x, y).0,
                    rgba(expected),
                    "cheek or protected mouth {asset} [{x},{y}] frame {frame}"
                );
            }
        }
    }
}

#[test]
#[ignore = "requires frozen before build and freshly generated Celine blush candidate"]
fn complete_celine_candidate_changes_only_reviewed_blush_pixels_for_six_additions() {
    let (set, profile) = blush_trial();
    let contract = read(root().join("docs/development/celine-native-tone-blush.json"));
    let before = root().join("generated/native-tone-build/characters/celine");
    let candidate = std::env::var_os("FOM_CELINE_BLUSH_BUILD")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("generated/celine-native-tone-blush"));
    let corrections: BTreeMap<_, _> = contract["regions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| (r["asset"].as_str().unwrap(), r))
        .collect();
    let (mut changed_files, mut changed_pixels, mut exact_old, mut exact_world, mut exact_other) =
        (0, 0, 0, 0, 0);
    let mut total_pixels = 0u64;
    for region in profile["regions"].as_array().unwrap() {
        let asset = region["asset"].as_str().unwrap();
        let original_path = before.join("original").join(asset);
        assert_eq!(
            format!("{:x}", Sha256::digest(fs::read(&original_path).unwrap())),
            region["source_sha256"].as_str().unwrap()
        );
        let original = image::open(original_path).unwrap().to_rgba8();
        let (w, h) = original.dimensions();
        let mut selected: BTreeMap<(u32, u32), usize> = BTreeMap::new();
        if let Some(row) = corrections.get(asset) {
            for component in row["components"].as_array().unwrap() {
                let color = rgba(component["color"].as_str().unwrap());
                let slot = contract["source_colors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .position(|c| rgba(c.as_str().unwrap()) == color)
                    .unwrap();
                let seed = &component["seed"];
                let seed = (
                    seed[0].as_u64().unwrap() as u32,
                    seed[1].as_u64().unwrap() as u32,
                );
                let mut seen = BTreeSet::from([seed]);
                let mut stack = vec![seed];
                while let Some((x, y)) = stack.pop() {
                    assert_eq!(original.get_pixel(x, y).0, color);
                    assert!(selected.insert((x, y), slot).is_none());
                    for p in [
                        x.checked_sub(1).map(|nx| (nx, y)),
                        y.checked_sub(1).map(|ny| (x, ny)),
                        Some((x + 1, y)),
                        Some((x, y + 1)),
                    ]
                    .into_iter()
                    .flatten()
                    {
                        if p.0 < w
                            && p.1 < h
                            && original.get_pixel(p.0, p.1).0 == color
                            && seen.insert(p)
                        {
                            stack.push(p);
                        }
                    }
                }
                assert_eq!(seen.len() as u64, component["pixels"].as_u64().unwrap());
            }
        }
        for (index, preset) in set["presets"].as_array().unwrap().iter().enumerate() {
            let id = preset["id"].as_str().unwrap();
            let old = before.join("variants").join(id).join(asset);
            let new = candidate.join("variants").join(id).join(asset);
            let old_bytes = fs::read(&old).unwrap();
            let new_bytes = fs::read(&new).unwrap();
            assert_eq!(
                fs::read(old.with_extension("meta.toml")).unwrap(),
                fs::read(new.with_extension("meta.toml")).unwrap()
            );
            if index < 4 {
                assert_eq!(old_bytes, new_bytes, "historical {id} {asset}");
                exact_old += 1;
                continue;
            }
            if !asset.contains("/Portraits/") {
                assert_eq!(old_bytes, new_bytes, "world {id} {asset}");
                exact_world += 1;
                continue;
            }
            if selected.is_empty() {
                assert_eq!(old_bytes, new_bytes, "unaffected {id} {asset}");
                exact_other += 1;
                continue;
            }
            let old = image::load_from_memory(&old_bytes).unwrap().to_rgba8();
            let new = image::load_from_memory(&new_bytes).unwrap().to_rgba8();
            assert_eq!(old.dimensions(), new.dimensions());
            for (x, y, p) in old.enumerate_pixels() {
                let q = new.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                total_pixels += 1;
                if let Some(slot) = selected.get(&(x, y)) {
                    assert_eq!(*p, *original.get_pixel(x, y));
                    assert_eq!(q.0, rgba(preset["colors"][14 + slot].as_str().unwrap()));
                    assert_ne!(p, q);
                    changed_pixels += 1;
                } else {
                    assert_eq!(p, q, "unrelated pixel {id} {asset} [{x},{y}]");
                }
            }
            changed_files += 1;
        }
    }
    assert_eq!(
        (
            changed_files,
            changed_pixels,
            exact_old,
            exact_world,
            exact_other
        ),
        (378, 15084, 1560, 1242, 720)
    );
    println!(
        "390 source-bound strips × ten palettes: {changed_files} corrected portrait PNGs / {changed_pixels} blush pixels; {exact_old} historical, {exact_world} world and {exact_other} unaffected native portrait PNGs byte-exact; {total_pixels} decoded affected pixels checked"
    );
}
