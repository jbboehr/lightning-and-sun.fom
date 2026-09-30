use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn asset(name: &str) -> String {
    let prefix = "summer";
    format!("assets/animations/NPCs/Eiland/Sprites/Summer/spr_npc_eiland_{prefix}_{name}.png")
}
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}
fn color(value: &Value) -> u32 {
    u32::from_str_radix(&value.as_str().unwrap()[1..7], 16).unwrap()
}

#[test]
#[ignore = "requires 147 local animations in extracted/eiland-summer-standard-study and the accepted earlier output baseline"]
fn eiland_summer_world_cover_moving_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-summer-standard-study");
    let baseline = root.join(
        "generated/characters-balor-valen-summer-eiland-spring-finish-trial/characters/eiland",
    );
    let profile_path = std::env::var_os("FOM_EILAND_SUMMER_WORLD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let set_path = std::env::var_os("FOM_EILAND_SUMMER_WORLD_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let profile = read(&profile_path);
    assert_eq!(profile["regions"].as_array().unwrap().len(), 147);
    assert_eq!(profile["source_colors"].as_array().unwrap().len(), 12);
    assert_eq!(profile["color_groups"].as_array().unwrap().len(), 7);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Source-grid landmarks separate exposed arms, fingers and neck from Summer
    // gold trim, pink cloth, eyes and hair. All five world skin shades are skin here.
    let landmarks = [
        ("idle_east", 0, 39, 33, 0x6C2859, false),
        ("idle_east", 0, 40, 34, 0xBA6A4C, true),
        ("idle_east", 0, 41, 35, 0xE9A980, true),
        ("idle_east", 0, 38, 39, 0x7D3B14, true),
        ("idle_east", 0, 40, 40, 0xBA6A4C, true),
        ("idle_east", 0, 40, 41, 0xDE8F5D, true),
        ("idle_east", 0, 34, 45, 0xBA6A4C, true),
        ("idle_east", 0, 35, 46, 0xE9A980, true),
        ("idle_east", 0, 35, 47, 0x7D3B14, true),
        ("idle_east", 0, 44, 45, 0xBA6A4C, true),
        ("idle_east", 0, 45, 46, 0xDE8F5D, true),
        ("idle_east", 0, 41, 36, 0xE9A980, true),
        ("idle_east", 0, 38, 36, 0xECF0E9, false),
        ("idle_east", 0, 40, 43, 0x724E80, false),
        ("idle_east", 0, 39, 42, 0xF9C94D, false),
        ("idle_east", 0, 39, 46, 0x533061, false),
        ("idle_east", 0, 36, 44, 0xBBB5C7, false),
        ("idle_east", 0, 37, 31, 0xFFFFFF, false),
        ("idle_north", 0, 32, 46, 0xE9A980, true),
        ("idle_north", 0, 33, 47, 0x7D3B14, true),
        ("idle_north", 0, 34, 45, 0xDE8F5D, true),
        ("idle_north", 0, 46, 46, 0xE9A980, true),
        ("idle_north", 0, 38, 42, 0xDB5C81, false),
        ("idle_north", 0, 39, 43, 0xA54E7F, false),
        ("idle_north", 0, 39, 45, 0xB475BA, false),
        ("idle_north", 0, 37, 52, 0xC1BDC8, false),
        ("idle_south", 0, 33, 45, 0xBA6A4C, true),
        ("idle_south", 0, 32, 46, 0xE9A980, true),
        ("idle_south", 0, 33, 47, 0x7D3B14, true),
        ("idle_south", 0, 45, 45, 0xDE8F5D, true),
        ("idle_south", 0, 46, 47, 0x7D3B14, true),
        ("idle_south", 0, 39, 40, 0xBA6A4C, true),
        ("idle_south", 0, 40, 41, 0xDE8F5D, true),
        ("idle_south", 0, 39, 43, 0x724E80, false),
        ("idle_south", 0, 38, 42, 0xF9C94D, false),
        ("idle_south", 0, 37, 51, 0xC1BDC8, false),
        ("walk_east", 1, 33, 45, 0xBA6A4C, true),
        ("walk_east", 1, 34, 46, 0xE9A980, true),
        ("walk_east", 1, 33, 47, 0x7D3B14, true),
        ("walk_east", 1, 45, 45, 0xDE8F5D, true),
        ("walk_east", 1, 46, 46, 0xE9A980, true),
        ("walk_east", 1, 46, 47, 0x7D3B14, true),
        ("walk_east", 1, 40, 41, 0xBA6A4C, true),
        ("walk_east", 1, 40, 42, 0xDE8F5D, true),
        ("walk_east", 1, 36, 44, 0xEDE0EF, false),
        ("walk_east", 1, 39, 44, 0xDA8B36, false),
        ("walk_east", 1, 39, 47, 0x533061, false),
        ("walk_east", 1, 42, 52, 0x756279, false),
        ("walk_north", 1, 33, 46, 0xBA6A4C, true),
        ("walk_north", 1, 34, 46, 0xDE8F5D, true),
        ("walk_north", 1, 33, 48, 0xBA6A4C, true),
        ("walk_north", 1, 44, 46, 0x7D3B14, true),
        ("walk_north", 1, 45, 45, 0xBA6A4C, true),
        ("walk_north", 1, 45, 47, 0xBA6A4C, true),
        ("walk_north", 1, 36, 45, 0x000000, false),
        ("walk_north", 1, 40, 46, 0xB475BA, false),
        ("walk_north", 1, 40, 42, 0xEDE0EF, false),
        ("walk_south", 1, 33, 46, 0xBA6A4C, true),
        ("walk_south", 1, 34, 46, 0xDE8F5D, true),
        ("walk_south", 1, 33, 48, 0xBA6A4C, true),
        ("walk_south", 1, 44, 46, 0x7D3B14, true),
        ("walk_south", 1, 45, 45, 0xBA6A4C, true),
        ("walk_south", 1, 45, 47, 0xBA6A4C, true),
        ("walk_south", 1, 36, 45, 0x000000, false),
        ("walk_south", 1, 39, 46, 0xF9C94D, false),
        ("walk_south", 1, 37, 52, 0x927D96, false),
    ];
    let skin = [0xE9A980, 0xDE8F5D, 0xBA6A4C, 0x9C5241, 0x7D3B14];
    let mut first_selection = None;
    let mut prior_files = 0;
    for r in profile["regions"].as_array().unwrap() {
        let a = r["asset"].as_str().unwrap();
        assert_eq!(
            r["source_sha256"],
            format!("{:x}", Sha256::digest(fs::read(original.join(a)).unwrap()))
        );
    }
    for r in profile["regions"].as_array().unwrap().iter().take(125) {
        let a = r["asset"].as_str().unwrap();
        for file in [a.to_owned(), a.replace(".png", ".meta.toml")] {
            assert_eq!(
                fs::read(original.join(&file)).unwrap(),
                fs::read(baseline.join("original").join(&file)).unwrap(),
                "previous source {file}"
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
        let targets = [0, 9, 10, 3, 11].map(|i| color(&preset["colors"][i]));
        let mut selection = vec![];
        let mut changed = 0;
        for (name, frames) in [
            ("idle_east", 1),
            ("idle_north", 1),
            ("idle_south", 1),
            ("walk_east", 4),
            ("walk_north", 4),
            ("walk_south", 4),
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
                // Full source-color inventory is independent of the component seeds.
                let index = skin.iter().position(|c| rgba(*c) == p.0);
                let expected = index.map_or(p.0, |i| rgba(targets[i]));
                assert_eq!(q.0, expected, "exact source/target {id} {name} [{x},{y}]");
                selection.push(index.is_some());
                if index.is_some() {
                    counts[x as usize / 80] += 1;
                    changed += 1;
                }
            }
            let expected_counts: &[usize] = match name {
                "idle_east" => &[38],
                "idle_north" => &[18],
                "idle_south" => &[46],
                "walk_east" => &[38, 45, 38, 38],
                "walk_north" => &[18, 15, 18, 15],
                "walk_south" => &[46, 43, 46, 43],
                _ => unreachable!(),
            };
            assert_eq!(counts, expected_counts, "skin inventory {name}");
        }
        assert_eq!(changed, 505);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(125) {
            let a = r["asset"].as_str().unwrap();
            for file in [a.to_owned(), a.replace(".png", ".meta.toml")] {
                assert_eq!(
                    fs::read(output.join(&file)).unwrap(),
                    fs::read(baseline.join("variants").join(id).join(&file)).unwrap(),
                    "previous output changed {id} {file}"
                );
                prior_files += 1;
            }
        }
    }
    assert_eq!(prior_files, 1250);
}
