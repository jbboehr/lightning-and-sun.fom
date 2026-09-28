use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn march_summer_specials_adds_five_pinned_regions_without_new_colors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let p = read(root.join("palettes/profiles/march-world-trial.json"));
    assert_eq!(p["regions"].as_array().unwrap().len(), 342);
    assert_eq!(p["source_colors"].as_array().unwrap().len(), 16);
    assert_eq!(p["color_groups"].as_array().unwrap().len(), 7);
    let expected: Vec<_> = [
        "hammer_east",
        "wipebrow_south",
        "work_sit_end_north",
        "work_sit_loop_north",
        "work_sit_start_north",
    ]
    .iter()
    .map(|n| json!(asset(n)))
    .collect();
    let actual: Vec<_> = p["regions"].as_array().unwrap()[259..264]
        .iter()
        .map(|r| r["asset"].clone())
        .collect();
    assert_eq!(actual, expected);
}

fn asset(name: &str) -> String {
    let prefix = "specialanimation_summer";
    format!("assets/animations/NPCs/March/Sprites/Summer/spr_npc_march_{prefix}_{name}.png")
}
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}
fn color(value: &Value) -> u32 {
    u32::from_str_radix(&value.as_str().unwrap()[1..7], 16).unwrap()
}

#[test]
#[ignore = "requires 342 local animations in extracted/march-winter-actions-study and the accepted earlier output baseline"]
fn march_summer_specials_cover_moving_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/march-winter-actions-study");
    let baseline =
        root.join("generated/characters-reina-juniper-march-summer-reading-trial/characters/march");
    let profile_path = std::env::var_os("FOM_MARCH_SUMMER_SPECIALS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/march-world-trial.json"));
    let set_path = std::env::var_os("FOM_MARCH_SUMMER_SPECIALS_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/march-world-trial.json"));
    let profile = read(&profile_path);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Source-art points distinguish the moving grips from wood/metal/effects,
    // and the tiny rear-neck opening from the hair above and shirt below.
    let landmarks = [
        ("work_sit_loop_north", 0, 35, 38, 0xD37A57, true),
        ("work_sit_loop_north", 0, 36, 41, 0xE8B271, true),
        ("work_sit_loop_north", 0, 37, 41, 0x4F626B, false),
        ("wipebrow_south", 5, 38, 51, 0xEEDDA5, true),
        ("hammer_east", 1, 37, 51, 0xD37A57, true),
        ("hammer_east", 0, 42, 46, 0xEEDDA5, true),
        ("hammer_east", 0, 43, 47, 0x7D3B14, true),
        ("hammer_east", 0, 45, 45, 0xE3B57F, false),
        ("hammer_east", 0, 47, 45, 0xBB8151, false),
        ("hammer_east", 0, 48, 45, 0x936244, false),
        ("hammer_east", 0, 55, 44, 0x6099A8, false),
        ("hammer_east", 1, 33, 36, 0xEEDDA5, true),
        ("hammer_east", 1, 35, 36, 0x7D3B14, true),
        ("hammer_east", 1, 36, 39, 0xD37A57, true),
        ("hammer_east", 1, 34, 34, 0xBB8151, false),
        ("hammer_east", 1, 39, 27, 0xC5E3E4, false),
        ("hammer_east", 3, 45, 45, 0xEEDDA5, true),
        ("hammer_east", 3, 50, 44, 0xD37A57, true),
        ("hammer_east", 3, 45, 47, 0xF8FF79, false),
        ("hammer_east", 6, 47, 46, 0xE8B271, true),
        ("hammer_east", 6, 41, 44, 0x94715E, false),
        ("wipebrow_south", 1, 41, 34, 0xD37A57, true),
        ("wipebrow_south", 1, 43, 34, 0xEEDDA5, true),
        ("wipebrow_south", 1, 42, 36, 0x7D3B14, true),
        ("wipebrow_south", 1, 32, 47, 0xEEDDA5, true),
        ("wipebrow_south", 1, 41, 40, 0x9E2626, false),
        ("wipebrow_south", 1, 38, 35, 0xD37A57, true),
        ("wipebrow_south", 5, 45, 46, 0xEEDDA5, true),
        ("wipebrow_south", 5, 32, 47, 0xEEDDA5, true),
        ("work_sit_loop_north", 0, 39, 39, 0xD37A57, true),
        ("work_sit_loop_north", 6, 40, 39, 0xD37A57, true),
        ("work_sit_loop_north", 0, 38, 37, 0x612934, false),
        ("work_sit_start_north", 0, 34, 41, 0x7D3B14, true),
        ("work_sit_start_north", 0, 33, 43, 0xE8B271, true),
        ("work_sit_end_north", 1, 46, 41, 0xD37A57, true),
        ("work_sit_end_north", 1, 40, 42, 0x9DAEB7, false),
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
    for r in profile["regions"].as_array().unwrap().iter().take(259) {
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
            ("hammer_east", 7),
            ("wipebrow_south", 6),
            ("work_sit_end_north", 2),
            ("work_sit_loop_north", 7),
            ("work_sit_start_north", 1),
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
            if name.starts_with("work_sit") {
                assert_eq!(
                    parsed["asset_properties"]["offset"]["horizontal"].as_str(),
                    Some("Middle")
                );
            } else {
                assert_eq!(
                    parsed["asset_properties"]["offset"]["horizontal"].as_float(),
                    Some(40.0)
                );
            }
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
        assert_eq!(changed, 894);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(259) {
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
    assert_eq!(prior_files, 2590);
}
