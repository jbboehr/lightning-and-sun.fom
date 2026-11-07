use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn rgba(text: &str) -> [u8; 4] {
    let mut result = [255; 4];
    for (i, c) in result.iter_mut().enumerate().take((text.len() - 1) / 2) {
        *c = u8::from_str_radix(&text[1 + i * 2..3 + i * 2], 16).unwrap();
    }
    result
}
fn selected() -> (Value, Value) {
    let set = read(
        std::env::var_os("FOM_CELINE_FACE_SET")
            .map(PathBuf::from)
            .unwrap_or_else(|| root().join("palettes/sets/celine-native-tones.json")),
    );
    let p = set["presets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == "player_35")
        .unwrap();
    let path = p
        .get("profile")
        .unwrap_or(&set["profile"])
        .as_str()
        .unwrap();
    let profile = read(root().join("palettes/sets").join(path));
    (set, profile)
}

#[test]
fn celine_face_override_preserves_historical_choices_and_skin_roles() {
    let (set, profile) = selected();
    let base = read(root().join("palettes/profiles/celine-native-tone-blush.json"));
    let historical = read(root().join("palettes/sets/celine-world-trial.json"));
    let donors = read(root().join("palettes/catalog/native-tones.json"));
    assert_eq!(set["profile"], "../profiles/celine-native-tone-blush.json");
    assert_eq!(
        &profile["source_colors"].as_array().unwrap()[..17],
        base["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &profile["source_colors"].as_array().unwrap()[17..],
        &[
            json!("#C77146"),
            json!("#742F23"),
            json!("#DC8E4F"),
            json!("#F3AC87"),
            json!("#FFE1C5"),
            json!("#BA5E3F")
        ]
    );
    assert_eq!(
        &profile["color_groups"].as_array().unwrap()[..10],
        base["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        &profile["color_groups"].as_array().unwrap()[10..],
        &[
            json!(["#C77146"]),
            json!(["#742F23"]),
            json!(["#DC8E4F"]),
            json!(["#F3AC87"]),
            json!(["#FFE1C5"]),
            json!(["#BA5E3F"])
        ]
    );
    assert_eq!(profile["regions"].as_array().unwrap().len(), 390);
    for (region, old) in profile["regions"]
        .as_array()
        .unwrap()
        .iter()
        .zip(base["regions"].as_array().unwrap())
    {
        for key in ["asset", "source_sha256", "size"] {
            assert_eq!(region[key], old[key]);
        }
        let oldseeds = old["seeds"].as_array().unwrap();
        let seeds = region["seeds"].as_array().unwrap();
        assert_eq!(&seeds[..oldseeds.len()], oldseeds);
        if !region["asset"].as_str().unwrap().contains("/Portraits/") {
            assert_eq!(seeds, oldseeds);
        }
    }
    let cosmetics = [
        ["#EAA092FF", "#F1AAA2FF", "#D05E67FF"],
        ["#DCA07BFF", "#E6AA8AFF", "#BE6258FF"],
        ["#C98476FF", "#D48E84FF", "#B9575FFF"],
        ["#B85F55FF", "#C66E65FF", "#A2474AFF"],
        ["#A35353FF", "#B26363FF", "#84363FFF"],
        ["#84434DFF", "#96535FFF", "#6E2F3DFF"],
    ];
    for (i, p) in set["presets"].as_array().unwrap().iter().enumerate() {
        if i < 4 {
            let old = &historical["presets"][i];
            assert!(p.get("profile").is_none());
            assert_eq!(p["id"], old["id"]);
            assert_eq!(p["label"], old["label"]);
            assert_eq!(
                &p["colors"].as_array().unwrap()[..14],
                old["colors"].as_array().unwrap()
            );
            assert_eq!(
                &p["colors"].as_array().unwrap()[14..],
                &[json!("#F4AEAE"), json!("#FCB0B6"), json!("#E6687A")]
            );
        } else {
            assert_eq!(p["profile"], "../profiles/celine-native-tone-face.json");
            assert_eq!(p["colors"].as_array().unwrap().len(), 23);
            for (slot, role) in [0, 1, 2, 3, 3, 2, 3, 0, 1, 3, 2, 0, 1, 2]
                .into_iter()
                .enumerate()
            {
                assert_eq!(p["colors"][slot], donors["presets"][i - 4]["colors"][role]);
            }
            assert_eq!(
                &p["colors"].as_array().unwrap()[14..17],
                &cosmetics[i - 4].map(|x| json!(x))
            );
            assert_eq!(p["colors"][17], p["colors"][2]);
            assert_eq!(p["colors"][18], p["colors"][3]);
            assert_eq!(p["colors"][19], p["colors"][2]);
            assert_eq!(p["colors"][20], p["colors"][1]);
            assert_eq!(p["colors"][21], p["colors"][0]);
            assert_eq!(p["colors"][22], p["colors"][2]);
        }
    }
}

#[test]
#[ignore = "requires retained Celine original portrait corpus"]
fn celine_mouth_eye_edges_and_visible_blush_follow_the_selected_deep_tone() {
    let (set, mut profile) = selected();
    let temp = tempfile::tempdir().unwrap();
    let source = root().join("generated/native-tone-build/characters/celine/original");
    let names = ["blush", "blush_closed_eyes", "neutral", "blush_special"];
    let assets:Vec<_>=names.iter().map(|n|format!("assets/animations/NPCs/Celine/Portraits/Autumn/Garden/spr_portrait_celine_autumn_garden_{n}.png")).collect();
    profile["regions"]
        .as_array_mut()
        .unwrap()
        .retain(|r| assets.iter().any(|a| r["asset"] == *a));
    let input = temp.path().join("input");
    for a in &assets {
        let out = input.join(a);
        fs::create_dir_all(out.parent().unwrap()).unwrap();
        for p in [
            PathBuf::from(a),
            PathBuf::from(a.replace(".png", ".meta.toml")),
        ] {
            fs::copy(source.join(&p), input.join(p)).unwrap();
        }
    }
    let pp = temp.path().join("profile.json");
    fs::write(&pp, serde_json::to_vec(&profile).unwrap()).unwrap();
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
        serde_json::to_vec(&json!({"profile":pp,"rgba_map":map})).unwrap(),
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
    let points = [
        (0, 152, 86, "#B35A35", "#190705", "mouth lower skin edge"),
        (0, 154, 84, "#DC8E4F", "#24090A", "mouth side skin edge"),
        (0, 151, 85, "#FCB0B6", "#96535F", "outer lip light"),
        (0, 152, 84, "#E6687A", "#6E2F3D", "outer lip shade"),
        (0, 149, 72, "#B35A35", "#190705", "open eye skin rim"),
        (0, 141, 71, "#672115", "#190705", "open eye upper rim"),
        (1, 146, 72, "#C77146", "#24090A", "closed eyelid crease"),
        (1, 143, 74, "#742F23", "#190705", "closed eyelid rim"),
        (1, 143, 76, "#EDA77D", "#351012", "closed eyelid fill"),
        (0, 446, 83, "#BA5E3F", "#24090A", "open mouth skin contour"),
        (
            0,
            446,
            85,
            "#F3AC87",
            "#351012",
            "open mouth lower skin shade",
        ),
        (
            0,
            446,
            86,
            "#FFE1C5",
            "#4A171C",
            "open mouth lower skin highlight",
        ),
        (
            0,
            152,
            76,
            "#F4AEAE",
            "#84434D",
            "visible muted rose nose blush",
        ),
    ];
    let mut failures = Vec::new();
    for (i, x, y, source_color, target, label) in points {
        let before = image::open(input.join(&assets[i])).unwrap().to_rgba8();
        let after = image::open(output.join(&assets[i])).unwrap().to_rgba8();
        assert_eq!(
            before.get_pixel(x, y).0,
            rgba(source_color),
            "source {label}"
        );
        if after.get_pixel(x, y).0 != rgba(target) {
            failures.push(format!(
                "{label} [{x},{y}]: {:?} expected {:?}",
                after.get_pixel(x, y).0,
                rgba(target)
            ));
        }
    }
    for (i, x, y, color, label) in [
        (0, 142, 71, "#000000", "lash"),
        (0, 142, 75, "#619278", "iris"),
        (0, 140, 75, "#F5F5F5", "eye white"),
        (0, 144, 65, "#DC8E4F", "hair light"),
        (0, 142, 67, "#B35A35", "hair/brow shadow"),
        (0, 447, 83, "#A5462C", "mouth interior"),
        (0, 450, 83, "#A30A17", "tongue dark"),
        (0, 447, 84, "#FC708D", "tongue pink"),
        (0, 449, 83, "#772E24", "mouth interior dark"),
        (3, 144, 74, "#F5F5F5", "special eye white"),
    ] {
        let before = image::open(input.join(&assets[i])).unwrap().to_rgba8();
        let after = image::open(output.join(&assets[i])).unwrap().to_rgba8();
        assert_eq!(
            before.get_pixel(x, y).0,
            rgba(color),
            "source protected {label}"
        );
        assert_eq!(after.get_pixel(x, y).0, rgba(color), "protected {label}");
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
#[ignore = "requires frozen Celine blush build and freshly generated face candidate"]
fn complete_celine_face_candidate_preserves_all_unrelated_pixels_and_historical_outputs() {
    let (set, profile) = selected();
    let contract = read(root().join("docs/development/celine-native-tone-face.json"));
    let blush = read(root().join("docs/development/celine-native-tone-blush.json"));
    let source = root().join("generated/native-tone-build/characters/celine/original");
    let before = root().join("generated/celine-native-tone-blush");
    let after = root().join("generated/celine-native-tone-face");
    let mut allowed: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for row in blush["regions"]
        .as_array()
        .unwrap()
        .iter()
        .chain(contract["regions"].as_array().unwrap())
    {
        allowed
            .entry(row["asset"].as_str().unwrap().to_owned())
            .or_default()
            .extend(row["components"].as_array().unwrap().iter().cloned());
    }
    let (mut changed_files, mut changed_pixels, mut old_exact, mut world_exact, mut checked_pixels) =
        (0, 0, 0, 0, 0u64);
    for region in profile["regions"].as_array().unwrap() {
        let asset = region["asset"].as_str().unwrap();
        let original_bytes = fs::read(source.join(asset)).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&original_bytes)),
            region["source_sha256"].as_str().unwrap()
        );
        let original = image::load_from_memory(&original_bytes).unwrap().to_rgba8();
        let (w, h) = original.dimensions();
        let mut selection: BTreeMap<(u32, u32), usize> = BTreeMap::new();
        if let Some(components) = allowed.get(asset) {
            for c in components {
                let color = c
                    .get("color")
                    .map(|v| v.as_str().unwrap())
                    .unwrap_or_else(|| {
                        let colors = c["colors"].as_object().unwrap();
                        assert_eq!(colors.len(), 1);
                        colors.keys().next().unwrap()
                    });
                let color = rgba(color);
                let slot = profile["source_colors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .position(|v| rgba(v.as_str().unwrap()) == color)
                    .unwrap();
                let seed = &c["seed"];
                let seed = (
                    seed[0].as_u64().unwrap() as u32,
                    seed[1].as_u64().unwrap() as u32,
                );
                let mut seen = BTreeSet::from([seed]);
                let mut stack = vec![seed];
                while let Some((x, y)) = stack.pop() {
                    assert_eq!(original.get_pixel(x, y).0, color);
                    assert!(
                        selection.insert((x, y), slot).is_none(),
                        "overlapping reviewed components {asset} [{x},{y}]"
                    );
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
                assert_eq!(seen.len() as u64, c["pixels"].as_u64().unwrap());
            }
        }
        for (i, preset) in set["presets"].as_array().unwrap().iter().enumerate() {
            let id = preset["id"].as_str().unwrap();
            let old_path = before.join("variants").join(id).join(asset);
            let new_path = after.join("variants").join(id).join(asset);
            let old_bytes = fs::read(&old_path).unwrap();
            let new_bytes = fs::read(&new_path).unwrap();
            assert_eq!(
                fs::read(old_path.with_extension("meta.toml")).unwrap(),
                fs::read(new_path.with_extension("meta.toml")).unwrap()
            );
            if i < 4 {
                assert_eq!(old_bytes, new_bytes, "historical {id} {asset}");
                old_exact += 1;
                continue;
            }
            if !asset.contains("/Portraits/") {
                assert_eq!(old_bytes, new_bytes, "world {id} {asset}");
                world_exact += 1;
                continue;
            }
            assert!(
                !selection.is_empty(),
                "missing portrait facial review {asset}"
            );
            let old = image::load_from_memory(&old_bytes).unwrap().to_rgba8();
            let new = image::load_from_memory(&new_bytes).unwrap().to_rgba8();
            assert_eq!(old.dimensions(), new.dimensions());
            for (x, y, p) in old.enumerate_pixels() {
                let q = new.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                checked_pixels += 1;
                if let Some(slot) = selection.get(&(x, y)) {
                    assert_eq!(q.0, rgba(preset["colors"][*slot].as_str().unwrap()));
                    assert_ne!(
                        p, q,
                        "selected correction did not change {id} {asset} [{x},{y}]"
                    );
                    changed_pixels += 1;
                } else {
                    assert_eq!(
                        p, q,
                        "unrelated material or previously mapped skin {id} {asset} [{x},{y}]"
                    );
                }
            }
            changed_files += 1;
        }
    }
    assert_eq!((changed_files, old_exact, world_exact), (1098, 1560, 1242));
    assert_eq!(
        changed_pixels,
        (contract["counts"]["added_selected_pixels"]
            .as_u64()
            .unwrap()
            + 2514)
            * 6
    );
    println!(
        "390 strips × ten choices: {changed_files} corrected PNGs / {changed_pixels} facial pixels; {old_exact} historical and {world_exact} new-tone world PNGs byte-exact; metadata/alpha/all unrelated pixels exact over {checked_pixels} decoded pixels"
    );
}
