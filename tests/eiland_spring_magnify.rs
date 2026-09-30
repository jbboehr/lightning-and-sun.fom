use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn asset(name: &str) -> String {
    let prefix = "specialanimation_spring";
    format!("assets/animations/NPCs/Eiland/Sprites/Spring/spr_npc_eiland_{prefix}_{name}.png")
}
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}
fn color(value: &Value) -> u32 {
    u32::from_str_radix(&value.as_str().unwrap()[1..7], 16).unwrap()
}

#[test]
#[ignore = "requires 142 local animations in extracted/eiland-summer-actions-study and the accepted earlier output baseline"]
fn eiland_spring_magnify_cover_moving_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-summer-actions-study");
    let baseline = root
        .join("generated/characters-balor-valen-eiland-spring-specials-trial/characters/eiland");
    let profile_path = std::env::var_os("FOM_EILAND_SPRING_MAGNIFY_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let set_path = std::env::var_os("FOM_EILAND_SPRING_MAGNIFY_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let profile = read(&profile_path);
    assert_eq!(profile["regions"].as_array().unwrap().len(), 142);
    assert_eq!(profile["source_colors"].as_array().unwrap().len(), 12);
    assert_eq!(profile["color_groups"].as_array().unwrap().len(), 7);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Literal source-art landmarks distinguish moving fingers and fine face shading
    // from gold trim, uniform, eye details, magnifier rim, lens and handle.
    let landmarks = [
        ("magnify_start_south", 0, 39, 34, 0x9C5241, true),
        ("magnify_start_south", 0, 39, 35, 0xBA6A4C, true),
        ("magnify_start_south", 0, 33, 46, 0xE9A980, true),
        ("magnify_start_south", 0, 34, 47, 0x7D3B14, true),
        ("magnify_start_south", 0, 44, 46, 0xBA6A4C, true),
        ("magnify_start_south", 0, 37, 46, 0xBA6A4C, false),
        ("magnify_start_south", 0, 42, 47, 0xBA6A4C, false),
        ("magnify_start_south", 0, 39, 30, 0xE797AC, false),
        ("magnify_start_south", 1, 45, 41, 0xDE8F5D, true),
        ("magnify_start_south", 1, 46, 42, 0xE9A980, true),
        ("magnify_start_south", 1, 46, 43, 0xDE8F5D, true),
        ("magnify_start_south", 1, 45, 35, 0xBE6D44, false),
        ("magnify_start_south", 1, 46, 35, 0xFFF672, false),
        ("magnify_start_south", 1, 47, 37, 0x488DE1, false),
        ("magnify_start_south", 1, 42, 48, 0xBA6A4C, false),
        ("magnify_start_south", 2, 42, 43, 0xE9A980, true),
        ("magnify_start_south", 2, 42, 45, 0xBA6A4C, true),
        ("magnify_start_south", 2, 43, 45, 0xE9A980, true),
        ("magnify_start_south", 2, 33, 47, 0x7D3B14, true),
        ("magnify_start_south", 2, 42, 37, 0xBE6D44, false),
        ("magnify_start_south", 2, 43, 37, 0xFFF672, false),
        ("magnify_start_south", 2, 43, 39, 0xFFFFFF, false),
        ("magnify_start_south", 2, 42, 48, 0xBA6A4C, false),
        ("magnify_loop_south", 0, 40, 42, 0xE9A980, true),
        ("magnify_loop_south", 0, 41, 42, 0xDE8F5D, true),
        ("magnify_loop_south", 0, 40, 44, 0xBA6A4C, true),
        ("magnify_loop_south", 0, 34, 46, 0x7D3B14, true),
        ("magnify_loop_south", 0, 41, 41, 0xFFCF36, false),
        ("magnify_loop_south", 0, 40, 39, 0x488DE1, false),
        ("magnify_loop_south", 0, 42, 39, 0x74D2FF, false),
        ("magnify_loop_south", 0, 37, 45, 0xBA6A4C, false),
        ("magnify_loop_south", 0, 42, 46, 0xBA6A4C, false),
        ("magnify_end_south", 0, 46, 42, 0xDE8F5D, true),
        ("magnify_end_south", 0, 33, 46, 0x7D3B14, true),
        ("magnify_end_south", 0, 45, 34, 0xBE6D44, false),
        ("magnify_end_south", 0, 46, 34, 0xFFF672, false),
        ("magnify_end_south", 0, 42, 47, 0xBA6A4C, false),
        ("magnify_end_south", 1, 44, 45, 0xBA6A4C, true),
        ("magnify_end_south", 1, 33, 47, 0x7D3B14, true),
        ("magnify_end_south", 1, 37, 45, 0xBA6A4C, false),
        ("magnify_end_south", 1, 37, 36, 0xECF0E9, false),
        ("magnify_start_east", 0, 36, 46, 0xBA6A4C, true),
        ("magnify_start_east", 0, 45, 47, 0xDE8F5D, true),
        ("magnify_start_east", 0, 40, 41, 0xBA6A4C, true),
        ("magnify_start_east", 0, 43, 44, 0xDE8F5D, false),
        ("magnify_start_east", 0, 38, 46, 0xBA6A4C, false),
        ("magnify_start_east", 0, 43, 46, 0xBA6A4C, false),
        ("magnify_start_east", 1, 38, 41, 0xDE8F5D, true),
        ("magnify_start_east", 1, 39, 42, 0xE9A980, true),
        ("magnify_start_east", 1, 39, 43, 0xDE8F5D, true),
        ("magnify_start_east", 1, 37, 35, 0xBE6D44, false),
        ("magnify_start_east", 1, 38, 35, 0xFFF672, false),
        ("magnify_start_east", 1, 40, 37, 0x000000, false),
        ("magnify_start_east", 1, 40, 46, 0xBA6A4C, false),
        ("magnify_start_east", 2, 42, 45, 0xBA6A4C, true),
        ("magnify_start_east", 2, 45, 46, 0xDE8F5D, true),
        ("magnify_start_east", 2, 40, 33, 0x9C5241, true),
        ("magnify_start_east", 2, 40, 37, 0x000000, false),
        ("magnify_start_east", 2, 41, 37, 0xBE6D44, false),
        ("magnify_start_east", 2, 43, 46, 0xBA6A4C, false),
        ("magnify_loop_east", 0, 44, 42, 0xBA6A4C, true),
        ("magnify_loop_east", 0, 46, 42, 0xBA6A4C, true),
        ("magnify_loop_east", 0, 46, 43, 0xDE8F5D, true),
        ("magnify_loop_east", 0, 45, 44, 0xDE8F5D, true),
        ("magnify_loop_east", 0, 44, 36, 0xBE6D44, false),
        ("magnify_loop_east", 0, 45, 36, 0xFFF672, false),
        ("magnify_loop_east", 0, 46, 38, 0x488DE1, false),
        ("magnify_loop_east", 0, 38, 45, 0xBA6A4C, false),
        ("magnify_end_east", 0, 39, 43, 0xDE8F5D, true),
        ("magnify_end_east", 0, 45, 46, 0xDE8F5D, true),
        ("magnify_end_east", 0, 36, 36, 0xBE6D44, false),
        ("magnify_end_east", 0, 40, 46, 0xBA6A4C, false),
        ("magnify_end_east", 1, 35, 45, 0xE9A980, true),
        ("magnify_end_east", 1, 36, 45, 0xBA6A4C, true),
        ("magnify_end_east", 1, 45, 46, 0xDE8F5D, true),
        ("magnify_end_east", 1, 43, 36, 0xECF0E9, false),
        ("magnify_end_east", 1, 38, 45, 0xBA6A4C, false),
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
    for r in profile["regions"].as_array().unwrap().iter().take(112) {
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
            ("magnify_end_east", 2),
            ("magnify_end_south", 2),
            ("magnify_loop_east", 1),
            ("magnify_loop_south", 1),
            ("magnify_start_east", 3),
            ("magnify_start_south", 3),
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
                // Observed skin colors have 54 explicit uniform/gold-trim pixels excluded.
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
                "magnify_end_east" => &[17, 29],
                "magnify_end_south" => &[38, 33],
                "magnify_loop_east" => &[28],
                "magnify_loop_south" => &[25],
                "magnify_start_east" => &[37, 23, 24],
                "magnify_start_south" => &[42, 45, 37],
                _ => unreachable!(),
            };
            assert_eq!(counts, expected_counts, "skin inventory {name}");
        }
        assert_eq!(changed, 378);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(112) {
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
    assert_eq!(prior_files, 1120);
}

// Independent source-grid inventory: shared gold-trim and uniform pixels.
// East start [43,44] is an isolated DE8F5D chest-trim shade; the remaining
// exclusions are BA6A4C belt/cape trim beside the separately visible hands.
fn is_trim(name: &str, frame: u32, x: u32, y: u32) -> bool {
    let points: &[(u32, u32)] = match (name, frame) {
        ("magnify_end_east", 0) => &[(40, 46), (41, 46), (43, 46)],
        ("magnify_end_east", 1) => &[(38, 45), (40, 46), (41, 46), (43, 46)],
        ("magnify_end_south", 0) => &[
            (37, 45),
            (42, 45),
            (39, 46),
            (40, 46),
            (43, 46),
            (42, 47),
            (42, 48),
        ],
        ("magnify_end_south", 1) => &[
            (37, 45),
            (42, 45),
            (39, 46),
            (40, 46),
            (43, 46),
            (42, 47),
            (42, 48),
        ],
        ("magnify_loop_east", 0) => &[(38, 45), (40, 46), (41, 46), (43, 46)],
        ("magnify_loop_south", 0) => &[(37, 45), (39, 46), (40, 46), (42, 46), (42, 47), (42, 48)],
        ("magnify_start_east", 0) => &[(43, 44), (38, 46), (43, 46)],
        ("magnify_start_east", 1) => &[(40, 46), (41, 46), (43, 46)],
        ("magnify_start_east", 2) => &[(43, 46)],
        ("magnify_start_south", 0) => &[(37, 46), (42, 46), (42, 47), (42, 48)],
        ("magnify_start_south", 1) => &[
            (37, 45),
            (42, 45),
            (39, 46),
            (40, 46),
            (43, 46),
            (42, 47),
            (42, 48),
        ],
        ("magnify_start_south", 2) => &[(37, 45), (39, 46), (40, 46), (42, 47), (42, 48)],
        _ => &[],
    };
    points.contains(&(x, y))
}
