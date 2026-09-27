use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn march_world_preserves_portrait_regions_and_palette_roles() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let old = read(root.join("palettes/profiles/march-portraits.json"));
    let world = read(root.join("palettes/profiles/march-world-trial.json"));
    assert_eq!(world["regions"].as_array().unwrap().len(), 259);
    assert_eq!(
        &world["regions"].as_array().unwrap()[..181],
        old["regions"].as_array().unwrap()
    );
    assert_eq!(
        &world["source_colors"].as_array().unwrap()[..13],
        old["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &world["color_groups"].as_array().unwrap()[..4],
        old["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        &world["source_colors"].as_array().unwrap()[13..15],
        &[json!("#E8B271"), json!("#D37A57")]
    );
    assert_eq!(
        &world["color_groups"].as_array().unwrap()[4..6],
        &[json!(["#E8B271"]), json!(["#D37A57"])]
    );
    let old_set = read(root.join("palettes/sets/march-portraits-trial.json"));
    let new_set = read(root.join("palettes/sets/march-world-trial.json"));
    assert_eq!(new_set["profile"], "../profiles/march-world-trial.json");
    assert_eq!(new_set["presets"].as_array().unwrap().len(), 4);
    for (old, new) in old_set["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(new_set["presets"].as_array().unwrap())
    {
        assert_eq!(old["id"], new["id"]);
        assert_eq!(old["label"], new["label"]);
        assert_eq!(
            &new["colors"].as_array().unwrap()[..13],
            old["colors"].as_array().unwrap()
        );
        assert_eq!(
            &new["colors"].as_array().unwrap()[13..15],
            &[old["colors"][1].clone(), old["colors"][2].clone()]
        );
    }
    let style = read(root.join("palettes/stylized/march-world-trial.json"));
    let old_style = read(root.join("palettes/stylized/march-portraits.json"));
    for (source, target) in old_style["rgba_map"].as_object().unwrap() {
        assert_eq!(&style["rgba_map"][source], target);
    }
    for (i, source) in world["source_colors"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        assert_eq!(
            style["rgba_map"][source.as_str().unwrap()],
            new_set["presets"][0]["colors"][i]
        );
    }
    let expected: Vec<_> = [
        "idle_east",
        "idle_north",
        "idle_south",
        "walk_east",
        "walk_north",
        "walk_south",
    ]
    .iter()
    .map(|name| json!(asset(name)))
    .collect();
    let actual: Vec<_> = world["regions"].as_array().unwrap()[181..187]
        .iter()
        .map(|r| r["asset"].clone())
        .collect();
    assert_eq!(actual, expected);
}

fn asset(name: &str) -> String {
    format!("assets/animations/NPCs/March/Sprites/Spring/spr_npc_march_spring_{name}.png")
}
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}
fn color(value: &Value) -> u32 {
    u32::from_str_radix(&value.as_str().unwrap()[1..7], 16).unwrap()
}

#[test]
#[ignore = "requires 259 local animations in extracted/march-summer-reading-study and the accepted portrait output baseline"]
fn march_world_skin_materials_and_existing_portraits_are_preserved() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/march-summer-reading-study");
    let baseline = root.join("generated/characters-world-wedding-finish-trial/characters/march");
    let profile_path = std::env::var_os("FOM_MARCH_WORLD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/march-world-trial.json"));
    let set_path = std::env::var_os("FOM_MARCH_WORLD_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/march-world-trial.json"));
    let profile = read(&profile_path);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Coordinates were chosen from the source artwork: face ramp, nape,
    // moving fingertips, goggles, green cuffs, hair, apron and black eye.
    let landmarks = [
        ("idle_south", 0, 39, 38, 0xEEDDA5, true),
        ("idle_south", 0, 38, 38, 0xE8B271, true),
        ("idle_south", 0, 37, 38, 0xD37A57, true),
        ("idle_south", 0, 36, 38, 0x7D3B14, true),
        ("idle_south", 0, 33, 47, 0x7D3B14, true),
        ("idle_south", 0, 39, 40, 0xD37A57, true),
        ("idle_south", 0, 39, 30, 0xA83837, false),
        ("idle_south", 0, 39, 43, 0xD0EDB4, false),
        ("idle_south", 0, 39, 49, 0x36373A, false),
        ("idle_south", 0, 38, 37, 0x000000, false),
        ("idle_east", 0, 37, 37, 0xD37A57, true),
        ("idle_east", 0, 34, 38, 0x6F8893, false),
        ("idle_north", 0, 39, 40, 0xD37A57, true),
        ("idle_north", 0, 33, 47, 0x7D3B14, true),
        ("idle_north", 0, 40, 35, 0x6F8893, false),
        ("walk_east", 1, 32, 46, 0xEEDDA5, true),
        ("walk_east", 1, 33, 47, 0x7D3B14, true),
        ("walk_east", 1, 45, 46, 0xEEDDA5, true),
        ("walk_east", 1, 43, 44, 0x8EAE81, false),
        ("walk_north", 3, 33, 47, 0x7D3B14, true),
        ("walk_north", 3, 46, 48, 0xD37A57, true),
        ("walk_south", 1, 33, 48, 0xD37A57, true),
        ("walk_south", 1, 44, 46, 0x7D3B14, true),
    ];
    let skin = [0xEEDDA5, 0xE8B271, 0xD37A57, 0x7D3B14];
    let mut first_selection = None;
    let mut prior_files = 0;
    for r in profile["regions"].as_array().unwrap() {
        let a = r["asset"].as_str().unwrap();
        assert_eq!(
            r["source_sha256"],
            format!("{:x}", Sha256::digest(fs::read(original.join(a)).unwrap()))
        );
    }
    for r in profile["regions"].as_array().unwrap().iter().take(181) {
        let a = r["asset"].as_str().unwrap();
        for file in [a.to_owned(), a.replace(".png", ".meta.toml")] {
            assert_eq!(
                fs::read(original.join(&file)).unwrap(),
                fs::read(baseline.join("original").join(&file)).unwrap(),
                "portrait source {file}"
            );
            prior_files += 1;
        }
    }
    for preset in set["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let recipe = temp.path().join(format!("{id}.json"));
        let mapping: serde_json::Map<_, _> = profile["source_colors"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, c)| (c.as_str().unwrap().to_owned(), preset["colors"][i].clone()))
            .collect();
        fs::write(
            &recipe,
            serde_json::to_vec(
                &json!({"profile":profile_path.canonicalize().unwrap(),"rgba_map":mapping}),
            )
            .unwrap(),
        )
        .unwrap();
        let output = temp.path().join(id);
        let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["apply", "--input"])
            .arg(&original)
            .arg("--palette")
            .arg(&recipe)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let targets = [0, 13, 14, 10].map(|i| color(&preset["colors"][i]));
        let mut selection = vec![];
        let mut changed = 0;
        for name in [
            "idle_east",
            "idle_north",
            "idle_south",
            "walk_east",
            "walk_north",
            "walk_south",
        ] {
            let a = asset(name);
            let before = image::open(original.join(&a)).unwrap().to_rgba8();
            let after = image::open(output.join(&a)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), after.dimensions());
            assert_eq!(
                before.dimensions(),
                (if name.starts_with("idle") { 80 } else { 320 }, 80)
            );
            let meta = a.replace(".png", ".meta.toml");
            let bytes = fs::read(original.join(&meta)).unwrap();
            assert_eq!(bytes, fs::read(output.join(&meta)).unwrap());
            let parsed: toml::Value = toml::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap();
            assert_eq!(
                parsed["asset_properties"]["atlas"].as_str(),
                Some("Default")
            );
            assert_eq!(
                parsed["asset_properties"]["offset"]["horizontal"].as_str(),
                Some("Middle")
            );
            assert_eq!(
                parsed["asset_properties"]["offset"]["vertical"].as_float(),
                Some(54.0)
            );
            for &(case, f, x, y, c, selected) in &landmarks {
                if case != name {
                    continue;
                }
                assert_eq!(
                    before.get_pixel(f * 80 + x, y).0,
                    rgba(c),
                    "source {case} {f} [{x},{y}]"
                );
                let expected = if selected {
                    targets[skin.iter().position(|s| *s == c).unwrap()]
                } else {
                    c
                };
                assert_eq!(
                    after.get_pixel(f * 80 + x, y).0,
                    rgba(expected),
                    "material {id} {case} {f} [{x},{y}] skin={selected}"
                );
            }
            let mut counts = vec![0; before.width() as usize / 80];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                // All four observed skin shades are skin in these six strips.
                // Checking this independent inventory catches omitted tiny components.
                let index = skin.iter().position(|c| rgba(*c) == p.0);
                let expected = index.map_or(p.0, |i| rgba(targets[i]));
                assert_eq!(q.0, expected, "exact source/target {id} {name} [{x},{y}]");
                selection.push(index.is_some());
                if index.is_some() {
                    counts[x as usize / 80] += 1;
                    changed += 1;
                }
            }
            assert!(counts.iter().all(|n| *n > 0));
        }
        assert_eq!(changed, 536);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(181) {
            let a = r["asset"].as_str().unwrap();
            for file in [a.to_owned(), a.replace(".png", ".meta.toml")] {
                assert_eq!(
                    fs::read(output.join(&file)).unwrap(),
                    fs::read(baseline.join("variants").join(id).join(&file)).unwrap(),
                    "portrait changed {id} {file}"
                );
                prior_files += 1;
            }
        }
    }
    assert_eq!(prior_files, 1810);
}
