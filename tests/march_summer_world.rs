use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn march_summer_world_adds_six_pinned_regions_without_new_colors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let p = read(root.join("palettes/profiles/march-world-trial.json"));
    assert_eq!(p["regions"].as_array().unwrap().len(), 240);
    assert_eq!(p["source_colors"].as_array().unwrap().len(), 16);
    assert_eq!(p["color_groups"].as_array().unwrap().len(), 7);
    let expected: Vec<_> = [
        "idle_east",
        "idle_north",
        "idle_south",
        "walk_east",
        "walk_north",
        "walk_south",
    ]
    .iter()
    .map(|n| json!(asset(n)))
    .collect();
    let actual: Vec<_> = p["regions"].as_array().unwrap()[234..240]
        .iter()
        .map(|r| r["asset"].clone())
        .collect();
    assert_eq!(actual, expected);
}

fn asset(name: &str) -> String {
    let prefix = "summer";
    format!("assets/animations/NPCs/March/Sprites/Summer/spr_npc_march_{prefix}_{name}.png")
}
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}
fn color(value: &Value) -> u32 {
    u32::from_str_radix(&value.as_str().unwrap()[1..7], 16).unwrap()
}

#[test]
#[ignore = "requires 240 local animations in extracted/march-summer-world-study and the accepted earlier output baseline"]
fn march_summer_world_cover_moving_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/march-summer-world-study");
    let baseline = root.join("generated/characters-march-spring-injured-trial/characters/march");
    let profile_path = std::env::var_os("FOM_MARCH_SUMMER_WORLD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/march-world-trial.json"));
    let set_path = std::env::var_os("FOM_MARCH_SUMMER_WORLD_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/march-world-trial.json"));
    let profile = read(&profile_path);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Source-art landmarks distinguish exposed arms, fingertips, neckline and
    // small leg patches from the neighboring brown apron and shoulder straps.
    let landmarks = [
        ("idle_east", 0, 35, 47, 0x7D3B14, true),
        ("idle_east", 0, 36, 47, 0xEEDDA5, true),
        ("idle_east", 0, 38, 42, 0x94715E, false),
        ("idle_east", 0, 37, 44, 0x402A24, false),
        ("idle_east", 0, 40, 45, 0x94715E, false),
        ("idle_east", 0, 45, 46, 0xE8B271, true),
        ("idle_east", 0, 38, 51, 0xD37A57, true),
        ("idle_north", 0, 39, 40, 0xD37A57, true),
        ("idle_north", 0, 36, 44, 0x7D3B14, true),
        ("idle_north", 0, 33, 47, 0x7D3B14, true),
        ("idle_north", 0, 46, 47, 0x7D3B14, true),
        ("idle_north", 0, 39, 41, 0x94715E, false),
        ("idle_north", 0, 37, 51, 0xE8B271, true),
        ("idle_south", 0, 39, 42, 0xEEDDA5, true),
        ("idle_south", 0, 33, 47, 0x7D3B14, true),
        ("idle_south", 0, 46, 47, 0x7D3B14, true),
        ("idle_south", 0, 42, 51, 0xE8B271, true),
        ("idle_south", 0, 39, 46, 0x402A24, false),
        ("walk_east", 1, 33, 47, 0x7D3B14, true),
        ("walk_east", 1, 44, 46, 0x7D3B14, true),
        ("walk_east", 1, 46, 47, 0x7D3B14, true),
        ("walk_east", 1, 37, 46, 0x402A24, false),
        ("walk_east", 3, 36, 48, 0x7D3B14, true),
        ("walk_east", 3, 42, 52, 0xD37A57, true),
        ("walk_north", 1, 38, 50, 0xE8B271, true),
        ("walk_north", 1, 34, 48, 0x7D3B14, true),
        ("walk_north", 1, 39, 42, 0x94715E, false),
        ("walk_north", 3, 33, 47, 0x7D3B14, true),
        ("walk_north", 3, 45, 48, 0x7D3B14, true),
        ("walk_south", 1, 44, 46, 0x7D3B14, true),
        ("walk_south", 1, 37, 47, 0x402A24, false),
        ("walk_south", 3, 45, 48, 0x7D3B14, true),
        ("walk_south", 3, 37, 52, 0xE8B271, true),
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
    for r in profile["regions"].as_array().unwrap().iter().take(234) {
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
        assert_eq!(changed, 728);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(234) {
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
    assert_eq!(prior_files, 2340);
}
