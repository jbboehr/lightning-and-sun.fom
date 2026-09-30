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
#[ignore = "requires 162 local animations in extracted/eiland-summer-magnify-study and the accepted earlier output baseline"]
fn eiland_spring_actions_cover_moving_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-summer-magnify-study");
    let baseline =
        root.join("generated/characters-balor-valen-eiland-world-trial/characters/eiland");
    let profile_path = std::env::var_os("FOM_EILAND_SPRING_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let set_path = std::env::var_os("FOM_EILAND_SPRING_ACTIONS_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let profile = read(&profile_path);
    assert_eq!(profile["regions"].as_array().unwrap().len(), 162);
    assert_eq!(profile["source_colors"].as_array().unwrap().len(), 12);
    assert_eq!(profile["color_groups"].as_array().unwrap().len(), 7);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Literal source-art landmarks distinguish moving fingers and fine face shading
    // from gold trim, uniform, eye details and true red mouth interiors.
    let landmarks = [
        ("blink_east", 1, 40, 35, 0xE9A980, true),
        ("blink_east", 1, 40, 33, 0x9C5241, true),
        ("blink_east", 1, 38, 39, 0x7D3B14, true),
        ("blink_east", 1, 35, 47, 0x7D3B14, true),
        ("blink_east", 1, 43, 43, 0xBA6A4C, false),
        ("blink_east", 1, 40, 46, 0xBA6A4C, false),
        ("blink_south", 1, 42, 39, 0x7D3B14, true),
        ("blink_south", 1, 42, 48, 0xBA6A4C, false),
        ("sit_east", 0, 35, 47, 0x7D3B14, true),
        ("sit_east", 0, 38, 44, 0xBA6A4C, false),
        ("sit_east", 0, 38, 45, 0x6A3126, false),
        ("sit_south", 0, 34, 46, 0xDE8F5D, true),
        ("sit_south", 0, 45, 47, 0x7D3B14, true),
        ("sit_south", 0, 43, 47, 0xBA6A4C, false),
        ("sit_north", 0, 39, 44, 0xC1BDC8, false),
        ("sit_north", 0, 39, 46, 0x473C52, false),
        ("drink_east", 0, 40, 42, 0xE9A980, true),
        ("drink_east", 0, 39, 43, 0xBA6A4C, true),
        ("drink_east", 1, 40, 40, 0xDE8F5D, true),
        ("drink_east", 1, 40, 42, 0xBA6A4C, true),
        ("drink_east", 1, 39, 43, 0xBA6A4C, true),
        ("drink_east", 1, 41, 43, 0xDB5C81, false),
        ("drink_south", 1, 35, 40, 0xE9A980, true),
        ("drink_south", 1, 34, 42, 0xBA6A4C, true),
        ("drink_south", 1, 35, 43, 0x7D3B14, true),
        ("drink_south", 1, 43, 45, 0xBA6A4C, false),
        ("drink_north", 0, 46, 42, 0xBA6A4C, true),
        ("drink_north", 1, 44, 41, 0xBA6A4C, true),
        ("drink_north", 1, 35, 44, 0xF4F4F4, false),
        ("eat_east", 0, 43, 42, 0xE9A980, true),
        ("eat_east", 0, 38, 44, 0xBA6A4C, false),
        ("eat_east", 1, 44, 40, 0xBA6A4C, true),
        ("eat_east", 1, 45, 42, 0xBA6A4C, true),
        ("eat_east", 1, 40, 43, 0x7D3B14, false),
        ("eat_east", 2, 41, 36, 0x9E2626, false),
        ("eat_east", 2, 43, 39, 0xE9A980, true),
        ("eat_east", 3, 44, 43, 0xDE8F5D, true),
        ("eat_east", 4, 43, 44, 0xDE8F5D, true),
        ("eat_south", 0, 37, 46, 0xDE8F5D, true),
        ("eat_south", 1, 37, 44, 0xDE8F5D, false),
        ("eat_south", 1, 36, 49, 0xBA6A4C, true),
        ("eat_south", 2, 38, 40, 0xBA6A4C, true),
        ("eat_south", 2, 40, 36, 0x9E2626, false),
        ("eat_south", 3, 35, 43, 0x7D3B14, true),
        ("eat_south", 4, 34, 47, 0x7D3B14, true),
        ("eat_north", 1, 45, 42, 0xBA6A4C, true),
        ("eat_north", 1, 35, 45, 0xF4F4F4, false),
        ("blink_east", 0, 38, 36, 0xC2B9BE, false),
        ("blink_east", 0, 37, 36, 0x6C2859, false),
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
    for r in profile["regions"].as_array().unwrap().iter().take(84) {
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
            ("blink_east", 3),
            ("blink_south", 3),
            ("drink_east", 3),
            ("drink_north", 3),
            ("drink_south", 3),
            ("eat_east", 5),
            ("eat_north", 3),
            ("eat_south", 5),
            ("sit_east", 1),
            ("sit_north", 1),
            ("sit_south", 1),
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
                // Observed skin colors have 86 explicit uniform/gold-trim pixels excluded.
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
                "blink_east" => &[33, 40, 33],
                "blink_south" => &[39, 46, 39],
                "drink_east" => &[34, 40, 34],
                "drink_north" => &[2, 3, 2],
                "drink_south" => &[36, 45, 36],
                "eat_east" => &[29, 36, 24, 35, 29],
                "eat_north" => &[2, 3, 2],
                "eat_south" => &[35, 37, 30, 46, 34],
                "sit_east" => &[30],
                "sit_north" => &[0],
                "sit_south" => &[34],
                _ => unreachable!(),
            };
            assert_eq!(counts, expected_counts, "skin inventory {name}");
        }
        assert_eq!(changed, 868);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(84) {
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
    assert_eq!(prior_files, 840);
}

// Independent source-grid inventory: shared gold-trim and uniform pixels.
fn is_trim(name: &str, frame: u32, x: u32, y: u32) -> bool {
    let points: &[(u32, u32)] = match (name, frame) {
        ("blink_east", 0) => &[(43, 43), (38, 45), (40, 46), (41, 46), (43, 46)],
        ("blink_south", 0) => &[
            (37, 45),
            (42, 45),
            (39, 46),
            (40, 46),
            (43, 46),
            (42, 47),
            (42, 48),
        ],
        ("drink_south", 0) => &[(37, 44), (42, 44), (43, 45), (43, 46), (43, 47)],
        ("blink_east", 1) => &[(43, 43), (38, 45), (40, 46), (41, 46), (43, 46)],
        ("blink_south", 1) => &[
            (37, 45),
            (42, 45),
            (39, 46),
            (40, 46),
            (43, 46),
            (42, 47),
            (42, 48),
        ],
        ("drink_south", 1) => &[(37, 44), (42, 44), (43, 45), (43, 46), (43, 47)],
        ("blink_east", 2) => &[(43, 43), (38, 45), (40, 46), (41, 46), (43, 46)],
        ("blink_south", 2) => &[
            (37, 45),
            (42, 45),
            (39, 46),
            (40, 46),
            (43, 46),
            (42, 47),
            (42, 48),
        ],
        ("drink_south", 2) => &[(37, 44), (42, 44), (43, 45), (43, 46), (43, 47)],
        ("sit_east", 0) => &[(38, 44)],
        ("sit_south", 0) => &[(37, 44), (42, 44), (43, 45), (43, 46), (43, 47)],
        ("eat_east", 0) => &[(38, 44), (39, 44)],
        ("eat_east", 1) => &[(38, 44), (39, 44), (40, 43)],
        ("eat_east", 2) => &[(38, 44), (39, 44)],
        ("eat_south", 0) => &[(37, 44), (42, 44), (43, 45), (43, 46), (43, 47)],
        ("eat_south", 2) => &[(37, 44), (42, 44), (43, 45), (43, 46), (43, 47)],
        ("eat_south", 4) => &[(37, 44), (42, 44), (43, 45), (43, 46), (43, 47)],
        ("eat_south", 1) => &[(37, 44), (42, 45), (43, 46), (43, 47)],
        ("eat_south", 3) => &[(42, 45), (43, 46), (43, 47)],
        _ => &[],
    };
    points.contains(&(x, y))
}
