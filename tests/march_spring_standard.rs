use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn march_spring_standard_adds_five_pinned_regions_without_new_colors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let p = read(root.join("palettes/profiles/march-world-trial.json"));
    assert_eq!(p["regions"].as_array().unwrap().len(), 383);
    assert_eq!(p["source_colors"].as_array().unwrap().len(), 16);
    assert_eq!(p["color_groups"].as_array().unwrap().len(), 7);
    let expected: Vec<_> = [
        "action_east",
        "action_north",
        "action_south",
        "kiss_east",
        "sleep_east",
    ]
    .iter()
    .map(|n| json!(asset(n)))
    .collect();
    let actual: Vec<_> = p["regions"].as_array().unwrap()[198..203]
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
#[ignore = "requires 383 local animations in extracted/march-beach-swim-study and the accepted earlier output baseline"]
fn march_spring_standard_cover_moving_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/march-beach-swim-study");
    let baseline =
        root.join("generated/characters-reina-juniper-march-actions-trial/characters/march");
    let profile_path = std::env::var_os("FOM_MARCH_SPRING_STANDARD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/march-world-trial.json"));
    let set_path = std::env::var_os("FOM_MARCH_SPRING_STANDARD_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/march-world-trial.json"));
    let profile = read(&profile_path);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Source-art coordinates cover extended hands, deep fingertips, eyelids,
    // the sleep hand/face boundary, goggles, cuffs, apron, hair and boots.
    let landmarks = [
        ("action_east", 1, 47, 43, 0xEEDDA5, true),
        ("action_east", 1, 46, 43, 0xE8B271, true),
        ("action_east", 1, 48, 45, 0x7D3B14, true),
        ("action_east", 1, 49, 45, 0xD37A57, true),
        ("action_east", 1, 45, 44, 0x8EAE81, false),
        ("action_east", 1, 42, 37, 0x000000, false),
        ("action_east", 1, 41, 37, 0xECF0E9, false),
        ("action_north", 1, 35, 44, 0xEEDDA5, true),
        ("action_north", 1, 35, 46, 0x7D3B14, true),
        ("action_north", 1, 39, 39, 0xD37A57, true),
        ("action_north", 1, 40, 43, 0xD0EDB4, false),
        ("action_north", 1, 40, 34, 0x6F8893, false),
        ("action_south", 2, 37, 45, 0xEEDDA5, true),
        ("action_south", 2, 36, 45, 0xE8B271, true),
        ("action_south", 2, 38, 47, 0x7D3B14, true),
        ("action_south", 2, 45, 46, 0x7D3B14, true),
        ("action_south", 2, 39, 46, 0x4F626B, false),
        ("action_south", 2, 39, 30, 0xA83837, false),
        ("kiss_east", 2, 42, 35, 0xE8B271, true),
        ("kiss_east", 2, 46, 36, 0x000000, false),
        ("kiss_east", 2, 35, 44, 0xD37A57, true),
        ("kiss_east", 2, 36, 45, 0xEEDDA5, true),
        ("kiss_east", 2, 36, 46, 0x7D3B14, true),
        ("kiss_east", 2, 40, 40, 0x000000, false),
        ("sleep_east", 0, 43, 39, 0xE8B271, true),
        ("sleep_east", 0, 44, 39, 0x7D3B14, true),
        ("sleep_east", 0, 43, 40, 0xEEDDA5, true),
        ("sleep_east", 0, 42, 42, 0xE8B271, true),
        ("sleep_east", 0, 38, 37, 0x000000, false),
        ("sleep_east", 0, 42, 45, 0x6F8893, false),
        ("sleep_east", 0, 39, 53, 0xECF0E9, false),
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
    for r in profile["regions"].as_array().unwrap().iter().take(198) {
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
        for (name, frames) in [
            ("action_east", 7),
            ("action_north", 7),
            ("action_south", 7),
            ("kiss_east", 4),
            ("sleep_east", 1),
        ] {
            let a = asset(name);
            let before = image::open(original.join(&a)).unwrap().to_rgba8();
            let after = image::open(output.join(&a)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), after.dimensions());
            assert_eq!(before.dimensions(), (frames * 80, 80));
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
                // All four observed skin shades are skin in these five strips.
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
        assert_eq!(changed, 877);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(198) {
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
    assert_eq!(prior_files, 1980);
}
