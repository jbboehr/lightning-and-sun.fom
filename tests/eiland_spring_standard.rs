use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn asset(name: &str) -> String {
    let prefix = "spring";
    format!("assets/animations/NPCs/Eiland/Sprites/Spring/spr_npc_eiland_{prefix}_{name}.png")
}
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}
fn color(value: &Value) -> u32 {
    u32::from_str_radix(&value.as_str().unwrap()[1..7], 16).unwrap()
}

#[test]
#[ignore = "requires 172 local animations in extracted/eiland-autumn-world-study and the accepted earlier output baseline"]
fn eiland_spring_standard_cover_moving_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-autumn-world-study");
    let baseline =
        root.join("generated/characters-balor-valen-eiland-spring-actions-trial/characters/eiland");
    let profile_path = std::env::var_os("FOM_EILAND_SPRING_STANDARD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let set_path = std::env::var_os("FOM_EILAND_SPRING_STANDARD_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let profile = read(&profile_path);
    assert_eq!(profile["regions"].as_array().unwrap().len(), 172);
    assert_eq!(profile["source_colors"].as_array().unwrap().len(), 12);
    assert_eq!(profile["color_groups"].as_array().unwrap().len(), 7);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Literal source-art landmarks distinguish moving fingers and fine face shading
    // from gold trim, uniform, eye details and actual black mouth/eye details.
    // At North action [34,44], the Summer source exposes the full hand;
    // Spring leaves its upper edge visible beside the cape. Keep that skin mapped.
    let landmarks = [
        ("action_east", 0, 41, 34, 0x9C5241, true),
        ("action_east", 0, 40, 47, 0xE9A980, true),
        ("action_east", 0, 40, 48, 0x7D3B14, true),
        ("action_east", 0, 35, 47, 0xBA6A4C, false),
        ("action_east", 0, 44, 47, 0xBA6A4C, false),
        ("action_east", 1, 48, 43, 0xE9A980, true),
        ("action_east", 1, 49, 43, 0xDE8F5D, true),
        ("action_east", 1, 49, 45, 0xBA6A4C, true),
        ("action_east", 1, 48, 45, 0x7D3B14, true),
        ("action_east", 1, 40, 45, 0xBA6A4C, false),
        ("action_east", 1, 36, 49, 0xBA6A4C, false),
        ("action_east", 1, 42, 46, 0xBA6A4C, false),
        ("action_east", 2, 44, 46, 0xBA6A4C, true),
        ("action_east", 2, 45, 46, 0x7D3B14, true),
        ("action_east", 2, 42, 46, 0xBA6A4C, false),
        ("action_east", 6, 45, 46, 0xDE8F5D, true),
        ("action_north", 0, 33, 43, 0xBA6A4C, true),
        ("action_north", 1, 45, 41, 0xBA6A4C, true),
        ("action_north", 1, 34, 44, 0xBA6A4C, true),
        ("action_north", 2, 44, 39, 0xBA6A4C, true),
        ("action_north", 2, 45, 40, 0x7D3B14, true),
        ("action_north", 2, 34, 44, 0xBA6A4C, true),
        ("action_north", 5, 46, 42, 0xBA6A4C, true),
        ("action_north", 6, 32, 46, 0xE9A980, true),
        ("action_south", 0, 45, 46, 0xBA6A4C, true),
        ("action_south", 0, 33, 48, 0xBA6A4C, true),
        ("action_south", 0, 37, 46, 0xBA6A4C, false),
        ("action_south", 1, 44, 45, 0xBA6A4C, true),
        ("action_south", 1, 45, 46, 0x7D3B14, true),
        ("action_south", 1, 44, 49, 0xBA6A4C, false),
        ("action_south", 2, 37, 47, 0xBA6A4C, true),
        ("action_south", 2, 35, 47, 0xBA6A4C, false),
        ("action_south", 4, 44, 50, 0xBA6A4C, false),
        ("action_south", 6, 46, 47, 0x7D3B14, true),
        ("kiss_east", 0, 34, 48, 0xBA6A4C, true),
        ("kiss_east", 0, 42, 46, 0xBA6A4C, false),
        ("kiss_east", 1, 41, 40, 0xDE8F5D, true),
        ("kiss_east", 1, 43, 46, 0xBA6A4C, false),
        ("kiss_east", 2, 43, 40, 0xBA6A4C, true),
        ("kiss_east", 2, 35, 46, 0xBA6A4C, true),
        ("kiss_east", 2, 37, 43, 0xBA6A4C, false),
        ("kiss_east", 2, 36, 49, 0xBA6A4C, false),
        ("kiss_east", 3, 42, 38, 0xE9A980, true),
        ("sleep_east", 0, 40, 33, 0x9C5241, true),
        ("sleep_east", 0, 44, 39, 0x7D3B14, true),
        ("sleep_east", 0, 43, 41, 0xE9A980, true),
        ("sleep_east", 0, 36, 45, 0xBA6A4C, false),
        ("sleep_east", 0, 38, 45, 0xBA6A4C, false),
        ("sleep_east", 0, 38, 46, 0x6A3126, false),
        ("sleep_east", 0, 37, 36, 0x6C2859, false),
        ("sleep_east", 0, 38, 37, 0x000000, false),
        ("action_east", 1, 40, 41, 0x927D96, false),
        ("action_north", 1, 39, 43, 0xC1BDC8, false),
        ("action_east", 0, 39, 37, 0xECF0E9, false),
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
    for r in profile["regions"].as_array().unwrap().iter().take(95) {
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
                // Observed skin colors have 95 explicit uniform/gold-trim pixels excluded.
                // This independent inventory catches omitted tiny components.
                let index = skin
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !is_trim(name, x / 80, x % 80, y));
                let expected = index.map_or(p.0, |i| rgba(targets[i]));
                assert_eq!(q.0, expected, "exact source/target {id} {name} [{x},{y}]");
                selection.push(index.is_some());
                if index.is_some() {
                    counts[x as usize / 80] += 1;
                    changed += 1;
                }
            }
            let expected_counts: &[usize] = match name {
                "action_east" => &[31, 32, 31, 32, 31, 31, 32],
                "action_north" => &[7, 2, 4, 2, 4, 5, 4],
                "action_south" => &[36, 35, 35, 35, 35, 36, 38],
                "kiss_east" => &[31, 35, 39, 39],
                "sleep_east" => &[37],
                _ => unreachable!(),
            };
            assert_eq!(counts, expected_counts, "skin inventory {name}");
        }
        assert_eq!(changed, 679);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(95) {
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
    assert_eq!(prior_files, 950);
}

// Independent source-grid inventory: shared gold-trim and uniform pixels.
fn is_trim(name: &str, frame: u32, x: u32, y: u32) -> bool {
    let points: &[(u32, u32)] = match (name, frame) {
        ("action_east", 0) => &[(35, 47), (44, 47)],
        ("action_east", 5) => &[(35, 47), (44, 47)],
        ("action_east", 1) => &[(40, 45), (42, 46), (43, 46), (45, 46), (37, 47), (36, 49)],
        ("action_east", 3) => &[(40, 45), (42, 46), (43, 46), (45, 46), (37, 47), (36, 49)],
        ("action_east", 2) => &[(40, 45), (42, 46), (37, 47), (36, 49)],
        ("action_east", 4) => &[(40, 45), (42, 46), (37, 47), (36, 49)],
        ("action_east", 6) => &[(38, 45), (40, 46), (41, 46), (43, 46)],
        ("action_south", 0) => &[
            (37, 46),
            (42, 46),
            (39, 47),
            (40, 47),
            (43, 47),
            (42, 48),
            (42, 49),
        ],
        ("action_south", 5) => &[
            (37, 46),
            (42, 46),
            (39, 47),
            (40, 47),
            (43, 47),
            (42, 48),
            (42, 49),
        ],
        ("action_south", 1) => &[
            (42, 45),
            (39, 46),
            (40, 46),
            (43, 46),
            (42, 47),
            (42, 48),
            (44, 49),
            (44, 50),
        ],
        ("action_south", 3) => &[
            (42, 45),
            (39, 46),
            (40, 46),
            (43, 46),
            (42, 47),
            (42, 48),
            (44, 49),
            (44, 50),
        ],
        ("action_south", 2) => &[
            (42, 45),
            (43, 46),
            (35, 47),
            (42, 47),
            (42, 48),
            (44, 49),
            (44, 50),
        ],
        ("action_south", 4) => &[
            (42, 45),
            (43, 46),
            (35, 47),
            (42, 47),
            (42, 48),
            (44, 49),
            (44, 50),
        ],
        ("action_south", 6) => &[
            (37, 45),
            (42, 45),
            (39, 46),
            (40, 46),
            (43, 46),
            (42, 47),
            (42, 48),
        ],
        ("kiss_east", 0) => &[(42, 46)],
        ("kiss_east", 1) => &[(39, 46), (43, 46)],
        ("kiss_east", 3) => &[(39, 46), (43, 46)],
        ("kiss_east", 2) => &[(37, 43), (39, 45), (44, 45), (41, 46), (42, 46), (36, 49)],
        ("sleep_east", 0) => &[(36, 45), (38, 45), (40, 46), (41, 46), (43, 46)],
        _ => &[],
    };
    points.contains(&(x, y))
}
